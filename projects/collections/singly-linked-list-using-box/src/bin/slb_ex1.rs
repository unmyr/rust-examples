#[derive(Debug, PartialEq)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(ref next) => {
                write!(f, "ListNode({:?}, _), {}", self.value, next)
            },
            None => write!(f, "ListNode({:?}, None)", self.value)
        }
    }
}

fn main() {
    // use std::mem::replace;
    let mut some_boxed_node_1 = Some(Box::new(ListNode::new(1)));
    let mut some_boxed_node_2 = Some(Box::new(ListNode::new(2)));
    let mut some_boxed_node_3 = Some(Box::new(ListNode::new(3)));

    let mut cur: Box<ListNode<_>>;
    let mut next_node: Box<ListNode<_>>;
    let mut prev_next: &mut Option<Box<ListNode<_>>>;

    // cur = replace(&mut some_boxed_node_3, None).unwrap();
    cur = some_boxed_node_3.take().unwrap();
    assert_eq!(some_boxed_node_3, None);
    next_node = cur;

    // cur = replace(&mut some_boxed_node_2, None).unwrap();
    cur = some_boxed_node_2.take().unwrap();
    assert_eq!(some_boxed_node_2, None);
    prev_next = &mut cur.next;
    *prev_next = Some(next_node);
    next_node = cur;

    // cur = replace(&mut some_boxed_node_1, None).unwrap();
    cur = some_boxed_node_1.take().unwrap();
    assert_eq!(some_boxed_node_1, None);
    prev_next = &mut cur.next;
    *prev_next = Some(next_node);

    println!("{}", cur);
}