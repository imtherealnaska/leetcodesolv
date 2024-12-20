use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::kth_largest_sum_in_a_binary_tree_med::TreeNode;

pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match root {
        None => return None,
        Some(r) => r,
    };

    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut level = 0;

    while !queue.is_empty() {
        let size = queue.len();
        let mut crrent_level_nodes = Vec::new();

        for _ in 0..size {
            let node = queue.pop_front().unwrap();
            crrent_level_nodes.push(node);
        }

        for node in &crrent_level_nodes {
            let node_ref = node.borrow();
            if let Some(left) = &node_ref.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node_ref.right {
                queue.push_back(right.clone());
            }
        }

        if level % 2 == 1 {
            let mut left = 0;
            let mut right = crrent_level_nodes.len() - 1;
            while left < right {
                let tmp = crrent_level_nodes[left].borrow().val;
                crrent_level_nodes[left].borrow_mut().val = crrent_level_nodes[right].borrow().val;
                crrent_level_nodes[right].borrow_mut().val = tmp;
                left += 1;
                right -= 1;
            }
        }
        level += 1;
    }
    Some(root)
}
