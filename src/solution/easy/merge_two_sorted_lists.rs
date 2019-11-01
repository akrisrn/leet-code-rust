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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}
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
