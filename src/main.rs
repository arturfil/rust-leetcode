// use helpers::select_random::select_random_problem;

mod linked_lists;

use linked_lists::reverse_list::reverse_list;
use linked_lists::ListNode;

fn main() {
    // select_random_problem();

    let mut head = ListNode::new(3);
    let mut first = head.next = head;

    reverse_list(head);
}


