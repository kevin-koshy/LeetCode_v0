// Definition for singly-linked list.

// use lc_83::{List,Link};
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
#[derive (Debug)]
struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(node) = cur {
            while let Some(next_node) = &mut node.next {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                } else {
                    break;
                }
            }
            cur = &mut node.next;
        }
        head
    }
}
fn main() {

    let mut tx = ListNode::new(1);
    push_val_to_node(& mut tx,2);
    push_val_to_node(& mut tx,2);
    push_val_to_node(& mut tx,4);
    push_val_to_node(& mut tx,5);

    println!("{:?}",Solution::delete_duplicates( Some(Box::new(tx))));
    // Solution::delete_duplicates( Some(Box::new(x)));



    }

