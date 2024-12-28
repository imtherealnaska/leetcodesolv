use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::kth_largest_sum_in_a_binary_tree_med::TreeNode;

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = VecDeque::from([root.clone().unwrap()]);
    let mut order = Vec::new();
    let mut swaps = 0;

    while !queue.is_empty() {
        order.extend(0..queue.len());
        order.sort_unstable_by_key(|&i| queue[i].borrow().val);

        for i in 0..order.len() {
            while order[i] != i {
                let j = order[i];
                order.swap(i, j);
                swaps += 1;
            }
        }

        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();

            if let Some(left) = node.left.as_ref() {
                queue.push_back(left.clone());
            }
            if let Some(right) = node.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
        order.clear();
    }
    swaps
}
