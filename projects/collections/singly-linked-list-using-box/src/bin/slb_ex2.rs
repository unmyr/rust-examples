use std::mem::replace;

#[derive(PartialEq)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(ref next) => {
                write!(f, "ListNode({:?}, _), {:?}", self.value, next)
            },
            None => write!(f, "ListNode({:?}, None)", self.value)
        }
    }
}

fn main() {
    let mut head: Option<Box<ListNode<u8>>> = None;

    // push_back(1)
    let some_boxed_node_1 = Some(Box::new(ListNode::new(2)));
    if head.is_none() {
        head = some_boxed_node_1;
        println!("1: {:?}", head);
    }

    // push_back(2)
    let some_boxed_node_2 = Some(Box::new(ListNode::new(2)));
    if head.is_some() {
        // let mut node1_cur = replace(&mut head, None).unwrap();
        let mut node1_cur = head.take().unwrap();
        if node1_cur.next.is_none() {
            let node1_cur_next = &mut node1_cur.next;
            *node1_cur_next = some_boxed_node_2;
        }
        head = Some(node1_cur);
        println!("2: {:?}", head);
    }

    // push_back(3)
    let some_boxed_node_3 = Some(Box::new(ListNode::new(3)));
    if head.is_some() {
        // let mut node1_cur = replace(&mut head, None).unwrap();
        let mut node1_cur = head.take().unwrap();
        if node1_cur.next.is_some() {
            let mut node2_cur = replace(&mut node1_cur.next, None).unwrap();
            if node2_cur.next.is_none() {
                let node2_cur_next = &mut node2_cur.next;
                *node2_cur_next = some_boxed_node_3;        
            }
            let node1_cur_next = &mut node1_cur.next;
            *node1_cur_next = Some(node2_cur);
        }
        head = Some(node1_cur);
        println!("3: {:?}", head);
    }
}
