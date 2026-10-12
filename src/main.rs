mod binary_trees;

use std::{cell::RefCell, rc::Rc};

use binary_trees::{invert_binary_tree, tree_node::TreeNode};

fn main() {
    // select_random_problem();
    //
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(8)));

    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    invert_binary_tree::invert_tree(Some(root));


}


