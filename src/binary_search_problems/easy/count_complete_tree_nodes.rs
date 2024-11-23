use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val: 0,
            left: None,
            right: None,
        }
    }
}

pub fn non_binary_seach_solution(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            return 1
                + non_binary_seach_solution(node.left)
                + non_binary_seach_solution(node.right);
        }
        None => 0,
    }
}
