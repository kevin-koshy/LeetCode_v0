#[derive(Debug)]
pub struct List{
    pub head: Link,
}

#[derive(Debug)]
pub enum Link{
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node{
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() -> List{
        List{head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node{
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match std::mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        todo!()
    }

}