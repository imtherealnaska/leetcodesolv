use std::{cell::RefCell, rc::Rc};

use super::kth_largest_sum_in_a_binary_tree_med::TreeNode;

///basically return true if two trees are flip of each other.
pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root1, root2) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(node1), Some(node2)) => {
            let node1 = node1.borrow();
            let node2 = node2.borrow();

            if node1.val != node2.val {
                return false;
            }

            (flip_equiv(node1.left.clone(), node2.left.clone())
                && flip_equiv(node1.right.clone(), node2.right.clone()))
                || (flip_equiv(node1.left.clone(), node2.right.clone())
                    && flip_equiv(node1.right.clone(), node2.left.clone()))
        }
    }
}

//         root
//     left        right
// val1  val2  rval1

fn create_sample_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let mut right = TreeNode::new(3);

    left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    right.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    Some(Rc::new(RefCell::new(root)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_equiv_works() {
        let tree1 = create_sample_tree();
        let tree2 = create_sample_tree();
        assert!(flip_equiv(tree1, tree2));

        let mut root = TreeNode::new(1);
        let mut left = TreeNode::new(3);
        let mut right = TreeNode::new(2);

        left.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let flipped_tree = Some(Rc::new(RefCell::new(root)));
        let original_tree = create_sample_tree();

        assert!(flip_equiv(original_tree, flipped_tree));
    }
}
