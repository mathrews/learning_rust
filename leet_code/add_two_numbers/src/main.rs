#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    {
        let mut new_node = None;
        let mut original_node = l1;

        while let Some(mut node) = original_node {
            original_node = node.next; // advancing the current node
            node.next = new_node;
            new_node = Some(node);
        }
    }
}
