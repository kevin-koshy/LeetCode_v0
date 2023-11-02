pub struct List{
    head: Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}

enum Node{
    elem: i32,
    next: Link,
}

