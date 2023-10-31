// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!()

    }
}

fn main() {
    let mut head = ListNode::new(1);
    let mut next_node = ListNode::new(123);
    head.next = Some(Box::new(next_node));
    let next_node2 = ListNode::new(3);
    next_node.next = Some(Box::new(next_node2));

    println!("{:?}", head.next.unwrap().val);
}

