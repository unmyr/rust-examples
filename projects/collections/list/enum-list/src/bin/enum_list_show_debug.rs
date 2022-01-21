use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let list_end = Rc::new(List::Nil);
    let list_3 = Rc::new(List::Cons(3, Rc::clone(&list_end)));
    let list_2 = Rc::new(List::Cons(2, Rc::clone(&list_3)));
    let list_head = Rc::new(List::Cons(1, Rc::clone(&list_2)));

    let s: Vec<Rc<List>> = vec![
        Rc::clone(&list_head),
        Rc::clone(&list_2),
        Rc::clone(&list_3),
        Rc::clone(&list_end),
    ];
    for elem in s {
        match *elem {
            List::Cons(value, ref next) => {
                println!("value: {}, next: {:?}", value, next);
            }
            List::Nil => {
                println!("nil");
            }
        }
    }
}
