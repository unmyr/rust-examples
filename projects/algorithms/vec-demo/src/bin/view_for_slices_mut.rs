use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let names = vec![
        Rc::new(RefCell::new(String::from("John"))),
        Rc::new(RefCell::new(String::from("Paul"))),
        Rc::new(RefCell::new(String::from("George"))),
        Rc::new(RefCell::new(String::from("Ringo")))
    ];
    println!("names={:?}", names);

    let a = &names[0..2];
    let b = &names[1..3];
    println!("a={:?} b={:?}", a, b);

    b[0].replace(String::from("XXX"));
    println!("a={:?} b={:?}", a, b);
    println!("names={:?}", names.iter().map(|x| x.borrow().clone()).collect::<Vec<String>>());
}