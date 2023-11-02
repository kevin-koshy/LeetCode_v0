// Definition for singly-linked list.

use lc_83::{List,Link};
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

// Function for pushing a value to a node
fn push_val_to_node(node: &mut ListNode, val: i32){
    if node.next.is_none(){
        node.next = Some(Box::new(ListNode::new(val)));
    }
    else{
        push_val_to_node(node.next.as_mut().unwrap(), val);
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("{:?}", head);

        let mut node = head;
        if node.is_none(){
            return None;
        }
        let mut curr = head;
        while  { curr != None; } {
            

        }
    }
}

fn main() {

    let mut x = ListNode::new(1);
    push_val_to_node(& mut x,2);
    push_val_to_node(& mut x,2);
    push_val_to_node(& mut x,4);
    push_val_to_node(& mut x,5);
    println!("{:?}", x);

    Solution::delete_duplicates( Some(Box::new(x)));
    }

