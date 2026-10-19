use std::{cell::RefCell, collections::VecDeque, rc::Rc, vec};
use super::tree_node::TreeNode;

/*
    Explanation: 
    You first get the root, put children in a stack? 

    put the value on a res stack
    
    get the len of that stack,
    while/for loop till that len is done
    put the children nodes on the stack
*/

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut queue = VecDeque::new();

    if let Some(root) = root {
        queue.push_back(root);
    }

   while !queue.is_empty() {
        let mut level = vec![];

        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();

            level.push(node.val);

            if let Some(left) = &node.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(right) = &node.right {
                queue.push_back(Rc::clone(right));
            }
        }
        res.push(level);
    }

    res
}

/*
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


                                   5
                               2        8
                            1    3    6     7
*/
