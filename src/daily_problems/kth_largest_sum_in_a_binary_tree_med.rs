use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    if root.is_none() {
        return -1;
    }

    let mut level_sums = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_sum: i64 = 0;

        for _ in 0..level_size {
            if let Some(node_rc) = queue.pop_front() {
                if let Some(node) = node_rc {
                    let node = node.borrow();
                    level_sum += node.val as i64;

                    if let Some(left) = &node.left {
                        queue.push_back(Some(Rc::clone(left)));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Some(Rc::clone(right)));
                    }
                }
            }
        }
        level_sums.push(level_sum);
    }

    if k as usize > level_sums.len() {
        return -1;
    }

    level_sums.sort_unstable_by(|a, b| b.cmp(a));
    level_sums[k as usize - 1]
}
