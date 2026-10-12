use std::{cell::RefCell, rc::Rc};

use super::tree_node::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = &root {
        let mut n = node.borrow_mut();
        let left = n.left.take();
        let right = n.right.take();

        n.left = invert_tree(right);
        n.right = invert_tree(left);
    }

    root
}
