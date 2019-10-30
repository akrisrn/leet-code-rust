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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut pair = vec![];
        let mut stack = vec![];
        stack.push(root.unwrap());
        while !stack.is_empty() {
            if let Some(node) = &stack.pop() {
                let node_borrow = node.borrow();
                if pair.contains(&node_borrow.val) {
                    return true;
                } else {
                    pair.push(k - node_borrow.val)
                }
                if let Some(right) = &node_borrow.right {
                    stack.push(Rc::clone(right))
                }
                if let Some(left) = &node_borrow.left {
                    stack.push(Rc::clone(left))
                }
            }
        }
        false
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
    let mut a = TreeNode::new(5);
    let mut b = TreeNode::new(3);
    let mut c = TreeNode::new(6);
    let d = TreeNode::new(2);
    let e = TreeNode::new(4);
    let f = TreeNode::new(7);

    b.left = Some(Rc::new(RefCell::new(d)));
    b.right = Some(Rc::new(RefCell::new(e)));
    c.right = Some(Rc::new(RefCell::new(f)));
    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    assert!(Solution::find_target(Some(Rc::new(RefCell::new(a))), 9));
}

#[test]
fn test_b() {
    let mut a = TreeNode::new(5);
    let mut b = TreeNode::new(3);
    let mut c = TreeNode::new(6);
    let d = TreeNode::new(2);
    let e = TreeNode::new(4);
    let f = TreeNode::new(7);

    b.left = Some(Rc::new(RefCell::new(d)));
    b.right = Some(Rc::new(RefCell::new(e)));
    c.right = Some(Rc::new(RefCell::new(f)));
    a.left = Some(Rc::new(RefCell::new(b)));
    a.right = Some(Rc::new(RefCell::new(c)));

    assert!(!Solution::find_target(Some(Rc::new(RefCell::new(a))), 28));
}
