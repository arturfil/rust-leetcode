use super::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut current = head;

    while let Some(mut node) = current {
        let mut temp = node.next.take();
        node.next = dummy.take();
        dummy = Some(node);
        current = temp.take();
    }

    dummy
}
