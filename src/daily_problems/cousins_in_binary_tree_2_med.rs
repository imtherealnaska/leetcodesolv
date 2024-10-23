use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

// NOT MY CODE: 

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
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

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = if let Some(root) = root {
        root
    } else {
        return None;
    };

    let mut level_sum = 0;

    let mut queue = vec![(root.clone(), 0)];

    while !queue.is_empty() {
        let mut new_queue = vec![];
        let mut new_level_sum = 0;

        for (node, sibling_sum) in queue.into_iter() {
            node.borrow_mut().val = level_sum - sibling_sum;

            let mut sibling_sum = 0;
            let mut non_leaf_children = vec![];

            // Process left child if it exists
            if let Some(child) = node.borrow().left.clone() {
                sibling_sum += child.borrow().val;
                non_leaf_children.push(child);
            }

            // Process right child if it exists
            if let Some(child) = node.borrow().right.clone() {
                sibling_sum += child.borrow().val;
                non_leaf_children.push(child);
            }

            // Add the sibling sum to the next level's sum
            new_level_sum += sibling_sum;

            // Add non-leaf children to the new queue with their sibling sum
            non_leaf_children
                .into_iter()
                .for_each(|node| new_queue.push((node, sibling_sum)));
        }

        // Update the queue for the next iteration
        queue = new_queue;
        // Update the level sum for the next iteration
        level_sum = new_level_sum;
    }

    // Return the modified root node
    Some(root)
}
