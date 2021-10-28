use std::cell::RefCell;
use std::rc::Rc;

use crate::Solution;

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_mirror(&root, &root)
    }

    fn is_mirror(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left_node), Some(right_node)) => {
                let left_borrow = left_node.borrow();
                let right_borrow = right_node.borrow();
                if left_borrow.val == right_borrow.val
                    && Solution::is_mirror(&left_borrow.right, &right_borrow.left)
                    && Solution::is_mirror(&left_borrow.left, &right_borrow.right)
                {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_symmetric_another(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        stack.push(root.clone());
        stack.push(root);
        while !stack.is_empty() {
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();
            match (left, right) {
                (Some(left_node), Some(right_node)) => {
                    let left_borrow = left_node.borrow();
                    let right_borrow = right_node.borrow();
                    if left_borrow.val != right_borrow.val {
                        return false;
                    }
                    stack.push(left_borrow.left.clone());
                    stack.push(right_borrow.right.clone());
                    stack.push(left_borrow.right.clone());
                    stack.push(right_borrow.left.clone());
                }
                (None, None) => {}
                _ => {
                    return false;
                }
            }
        }
        true
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
