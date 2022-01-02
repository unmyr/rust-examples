use std::rc::Rc;

/// # Examples
/// 
/// ```
/// use std::rc::Rc;
/// use enum_list::List::{Cons, Nil};
/// 
/// let a = Rc::new(Cons(10, Rc::new(Nil)));
/// ```
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::List::{Cons, Nil};

    #[test]
    fn test_share() {
        // 5 -> 10 -> Nil
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        assert_eq!(Rc::strong_count(&a), 1);

        // 3
        // ↓ 
        // 5 -> 10 -> Nil
        let _b = Cons(3, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 2);

        // 3
        // ↓ 
        // 5 -> 10 -> Nil
        // ↑
        // 4
        {
            let _c = Cons(4, Rc::clone(&a));
            assert_eq!(Rc::strong_count(&a), 3);
        }

        // 3
        // ↓ 
        // 5 -> 10 -> Nil
        assert_eq!(Rc::strong_count(&a), 2);
    }
}
