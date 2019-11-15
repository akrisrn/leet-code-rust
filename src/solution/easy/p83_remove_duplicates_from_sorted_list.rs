use crate::Solution;

/// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
///
/// Given a sorted linked list, delete all duplicates such that each element appear only once.
///
/// # Example
///
/// > Input: 1->1->2
/// > Output: 1->2
///
/// > Input: 1->1->2->3->3
/// > Output: 1->2->3
///
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}
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
    let mut b = ListNode::new(1);
    let c = ListNode::new(2);
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));

    let mut d = ListNode::new(1);
    let e = ListNode::new(2);
    d.next = Some(Box::new(e));

    assert_eq!(Solution::delete_duplicates(Some(Box::new(a))), Some(Box::new(d)))
}

#[test]
fn test_b() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(1);
    let mut c = ListNode::new(2);
    let mut d = ListNode::new(3);
    let e = ListNode::new(3);
    d.next = Some(Box::new(e));
    c.next = Some(Box::new(d));
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));

    let mut f = ListNode::new(1);
    let mut g = ListNode::new(2);
    let h = ListNode::new(3);
    g.next = Some(Box::new(h));
    f.next = Some(Box::new(g));

    assert_eq!(Solution::delete_duplicates(Some(Box::new(a))), Some(Box::new(f)))
}
