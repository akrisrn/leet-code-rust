use crate::Solution;

/// https://leetcode.com/problems/merge-two-sorted-lists/
///
/// Merge two sorted linked lists and return it as a new list.
/// The new list should be made by splicing together the nodes of the first two lists.
///
/// # Example
///
/// > Input: 1->2->4, 1->3->4
/// > Output: 1->1->2->3->4->4
///
impl Solution {
    // cant solve it, copy from others.
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let some_box_list = |val, next| Some(Box::new(ListNode { val, next }));
        match (l1, l2) {
            (Some(left_node), Some(right_node)) => {
                if left_node.val < right_node.val {
                    some_box_list(left_node.val, Self::merge_two_lists(left_node.next, Some(right_node)))
                } else {
                    some_box_list(right_node.val, Self::merge_two_lists(Some(left_node), right_node.next))
                }
            }
            (Some(left_node), _) => {
                some_box_list(left_node.val, Self::merge_two_lists(left_node.next, None))
            }
            (l1, l2 @ Some(_)) => {
                Self::merge_two_lists(l2, l1)
            }
            _ => None
        }
    }

    pub fn merge_two_lists_another(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = &mut dummy_head;
        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                head.as_mut()?.next = l2;
                break;
            } else if l2.is_none() {
                head.as_mut()?.next = l1;
                break;
            }
            head.as_mut()?.next = if l1.as_ref()?.val < l2.as_ref()?.val {
                let (origin, next) = Self::take_head(l1);
                l1 = origin;
                next
            } else {
                let (origin, next) = Self::take_head(l2);
                l2 = origin;
                next
            };
            head = &mut head.as_mut()?.next;
        }
        dummy_head?.next
    }

    #[inline(always)]
    fn take_head(mut l: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let l_next = l.as_mut().unwrap().next.take();
        let next = l.take();
        l = l_next;
        (l, next)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

#[test]
fn test_a() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let c = ListNode::new(4);
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));
    let a = Some(Box::new(a));

    let mut d = ListNode::new(1);
    let mut e = ListNode::new(3);
    let f = ListNode::new(4);
    e.next = Some(Box::new(f));
    d.next = Some(Box::new(e));
    let d = Some(Box::new(d));

    let mut g = ListNode::new(1);
    let mut h = ListNode::new(1);
    let mut i = ListNode::new(2);
    let mut j = ListNode::new(3);
    let mut k = ListNode::new(4);
    let l = ListNode::new(4);
    k.next = Some(Box::new(l));
    j.next = Some(Box::new(k));
    i.next = Some(Box::new(j));
    h.next = Some(Box::new(i));
    g.next = Some(Box::new(h));
    let g = Some(Box::new(g));

    assert_eq!(Solution::merge_two_lists(a, d), g);
}

#[test]
fn test_b() {
    assert_eq!(Solution::merge_two_lists(None, None), None);
}

#[test]
fn test_c() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let c = ListNode::new(4);
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));
    let a = Some(Box::new(a));

    assert_eq!(Solution::merge_two_lists(a.clone(), None), a);
}

#[test]
fn test_d() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let c = ListNode::new(4);
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));
    let a = Some(Box::new(a));

    assert_eq!(Solution::merge_two_lists(None, a.clone()), a);
}
