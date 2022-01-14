use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let names = vec![
        Rc::new(RefCell::new("John")),
        Rc::new(RefCell::new("Paul")),
        Rc::new(RefCell::new("George")),
        Rc::new(RefCell::new("Ringo"))
    ];
    println!("names={:?}", names);

    let a = &names[0..2];
    let b = &names[1..3];
    println!("a={:?} b={:?}", a, b);

    b[0].replace("XXX");
    println!("a={:?} b={:?}", a, b);
    println!(
        "names={:?}",
        names.iter().map(
            |s| <&str>::clone(&s.borrow())
        ).collect::<Vec<&str>>()
    );
}