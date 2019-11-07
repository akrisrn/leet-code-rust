use crate::Solution;
use std::rc::Rc;
use std::cell::RefCell;

/// https://leetcode.com/problems/symmetric-tree/
///
/// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
/// For example, this binary tree `[1, 2, 2, 3, 4, 4, 3]` is symmetric:
///
/// >     1
/// >    / \
/// >   2   2
/// >  / \ / \
/// > 3  4 4  3
///
/// But the following `[1, 2, 2, null, 3, null, 3]` is not:
///
/// >   1
/// >  / \
/// > 2   2
/// >  \   \
/// >  3    3
///
/// Note: Bonus points if you could solve it both recursively and iteratively.
///
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {}
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

#[test]
fn test_a() {
    let mut a = TreeNode::new(1);
    let mut b = TreeNode::new(2);
    let mut c = TreeNode::new(2);
    let d = TreeNode::new(3);
    let e = TreeNode::new(4);
    let f = TreeNode::new(4);
    let g = TreeNode::new(3);

    b.left = Some(Rc::new(RefCell::new(d)));
    b.right = Some(Rc::new(RefCell::new(e)));
    c.left = Some(Rc::new(RefCell::new(f)));
    c.right = Some(Rc::new(RefCell::new(g)));
    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(a)))));
}

#[test]
fn test_b() {
    let mut a = TreeNode::new(1);
    let mut b = TreeNode::new(2);
    let mut c = TreeNode::new(2);
    let d = TreeNode::new(3);
    let e = TreeNode::new(3);

    b.right = Some(Rc::new(RefCell::new(d)));
    c.right = Some(Rc::new(RefCell::new(e)));
    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    assert!(!Solution::is_symmetric(Some(Rc::new(RefCell::new(a)))));
}
