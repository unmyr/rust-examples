use std::fmt;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Nil => write!(f, "Nil"),
            List::Cons(value, ref next) => {
                write!(f, "Cons({}, {})", value, next)
            }
        }
    }
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
        println!("{}", elem);
    }
}
