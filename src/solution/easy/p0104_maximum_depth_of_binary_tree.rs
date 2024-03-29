use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;

/// https://leetcode.com/problems/maximum-depth-of-binary-tree/
///
/// Given a binary tree, find its maximum depth.
/// The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
/// Note: A leaf is a node with no children.
///
/// # Example
///
/// Given binary tree `[3, 9, 20, null, null, 15, 7]`,
///
/// >   3
/// >  / \
/// > 9  20
/// >   /  \
/// >  15   7
///
/// return its depth = 3.
///
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        match root {
            Some(node) => {
                Solution::rec(node, 1, &mut max);
            }
            None => {}
        }
        max
    }

    fn rec(node: Rc<RefCell<TreeNode>>, lv: i32, max: &mut i32) {
        if lv > *max {
            *max = lv
        }
        let node_borrow = node.borrow();
        if let Some(left) = &node_borrow.left {
            Solution::rec(Rc::clone(left), lv + 1, max);
        }
        if let Some(right) = &node_borrow.right {
            Solution::rec(Rc::clone(right), lv + 1, max)
        }
    }
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
    let mut a = TreeNode::new(3);
    let b = TreeNode::new(9);
    let mut c = TreeNode::new(20);
    let d = TreeNode::new(15);
    let e = TreeNode::new(7);

    c.left = Some(Rc::new(RefCell::new(d)));
    c.right = Some(Rc::new(RefCell::new(e)));
    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(a)))), 3);
}
