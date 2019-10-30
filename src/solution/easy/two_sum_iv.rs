use crate::Solution;
use std::rc::Rc;
use std::cell::RefCell;

/// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
///
/// Given a Binary Search Tree and a target number,
/// return true if there exist two elements in the BST such that their sum is equal to the given target.
///
/// # Example
///
/// > Input:
/// >     5
/// >    / \
/// >   3   6
/// >  / \   \
/// > 2   4   7
/// >
/// > Target = 9
/// >
/// > Output: True
///
/// > Input:
/// >     5
/// >    / \
/// >   3   6
/// >  / \   \
/// > 2   4   7
/// >
/// > Target = 28
/// >
/// > Output: False
///
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {}
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
