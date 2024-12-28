use std::{cell::RefCell, collections::VecDeque, i32, rc::Rc};

use crate::linked_list_binary::TreeNode;

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut results = vec![];

    if root.is_none() {
        return results;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_max = i32::MIN;

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node = node.borrow();
                level_max = level_max.max(node.val);

                if let Some(left) = node.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back(right);
                }
            }
        }
        results.push(level_max);
    }
    results
}
