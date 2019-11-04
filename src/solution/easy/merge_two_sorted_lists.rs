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
