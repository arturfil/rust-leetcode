mod binary_trees;
use std::{cell::RefCell, rc::Rc};

use binary_trees::{order_traversal::level_order, tree_node::TreeNode};

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(8)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(1)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(3)));
    let right_left = Rc::new(RefCell::new(TreeNode::new(6)));
    let right_right = Rc::new(RefCell::new(TreeNode::new(7)));

    left.borrow_mut().left = Some(left_left);
    left.borrow_mut().right = Some(left_right);
    right.borrow_mut().left = Some(right_left);
    right.borrow_mut().right = Some(right_right);
    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    level_order(Some(root));
}
/*

                                   5
                               2        8
                            1    3    6     7
*/
