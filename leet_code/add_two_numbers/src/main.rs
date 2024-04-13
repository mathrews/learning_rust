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

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}

#[test]
fn test_first() {
    let input1 = [2, 4, 3];
    let input2 = [5, 6, 4];

    assert_eq!(add_two_numbers(input1, input2), [7, 0, 8]);
}
