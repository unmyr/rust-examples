#[cfg(test)]
mod tests {
    #[test]
    fn test_option_update_if_let_some_ref_mut() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);
        if let Some(ref mut s1) = o1 {
            // Update inner string.
            s1.push_str(" world!");
        }
        assert_eq!(o1.as_deref(), Some("Hello world!"));

        let mut o1: Option<String> = None;
        assert_eq!(o1.is_none(), true);
        if let Some(ref mut s1) = o1 {
            // Update inner string.
            s1.push_str(" world!");
        }
        assert_eq!(o1, None);
    }

    #[test]
    fn test_option_update_if_let_ref_amp() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);
        if let Some(s1) = &mut o1 {
            // Update inner string.
            s1.push_str(" world!");
        }
        assert_eq!(o1, Some(String::from("Hello world!")));

        let mut o1: Option<String> = None;
        assert_eq!(o1.is_none(), true);
        if let Some(s1) = &mut o1 {
            // Update inner string.
            s1.push_str(" world!");
        }
        assert_eq!(o1, None);
    }

    #[test]
    fn test_option_update_as_mut() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);

        // Update inner string.
        o1.as_mut().map(|s| s.push_str(" world!"));
        assert_eq!(o1, Some(String::from("Hello world!")));

        let mut o1: Option<String> = None;
        assert_eq!(o1.is_none(), true);
        o1.as_mut().map(|s| s.push_str(" world!"));
        assert_eq!(o1, None);
    }

    #[test]
    fn test_option_replace() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);
        // Replace inner string.
        let old_o1 = o1.replace(String::from("Hello world!"));
        assert_eq!(o1, Some(String::from("Hello world!")));
        assert_eq!(old_o1, Some(String::from("Hello")));

        let mut o1: Option<String> = None;
        assert_eq!(o1.is_none(), true);
        let old_o1 = o1.replace(String::from("Hello world!"));
        assert_eq!(o1, Some(String::from("Hello world!")));
        assert_eq!(old_o1, None);
    }

    #[test]
    fn test_option_rc_decrement() {
        use std::rc::Rc;
        let mut o1 = Some(Rc::new(String::from("a")));
        let o2 = o1.as_ref().map(|s| Rc::clone(s));
        assert_eq!(Rc::strong_count(o1.as_ref().unwrap()), 2);

        let o3: &mut Option<_> = &mut o1;
        *o3 = None;
        assert_eq!(o1.as_ref(), None);
        assert_eq!(Rc::strong_count(o2.as_ref().unwrap()), 1);
    }

    #[test]
    fn test_option_refcell_cloned() {
        use std::cell::RefCell;

        let o1 = Some(RefCell::new(String::from("Hello")));
        assert_eq!(o1.is_some(), true);
        let o2: &Option<RefCell<String>> = &o1.as_ref().cloned();

        // Update inner string.
        o2.as_ref().map(|c| c.borrow_mut().push_str(" world!"));

        assert_eq!(
            o1.as_ref().unwrap().borrow().clone(),
            String::from("Hello")
        );
        assert_eq!(
            o2.as_ref().unwrap().borrow().clone(),
            String::from("Hello world!")
        );
    }

    #[test]
    fn test_rc_refcell_as_deref() {
        use std::rc::Rc;
        use std::cell::RefCell;
        let o1 = Some(Rc::new(RefCell::new(String::from("Hello"))));
        assert_eq!(
            o1.as_deref(),
            Some(&RefCell::new(String::from("Hello")))
        );
        assert_eq!(
            o1.as_deref().unwrap(),
            &RefCell::new(String::from("Hello"))
        );
    
        o1.as_deref().unwrap().borrow_mut().push_str(" world!");
        assert_eq!(
            o1.as_deref().unwrap().borrow().clone(),
            "Hello world!",
        );
    
        let o1: Option<Rc<RefCell<String>>> = None;
        assert_eq!(o1.as_deref(), None);
    }

    #[test]
    fn test_option_rc_refcell_cloned() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut o1 = Some(Rc::new(RefCell::new(String::from("Hello"))));
        assert_eq!(o1.is_some(), true);
        let o2 = o1.as_ref().cloned();
        assert_eq!(Rc::strong_count(o1.as_ref().unwrap()), 2);

        // Update inner string.
        o2.as_ref().map(|c| c.borrow_mut().push_str(" world!"));

        assert_eq!(
            o1.as_ref().unwrap().borrow().clone(),
            String::from("Hello world!")
        );
        assert_eq!(
            o2.as_ref().unwrap().borrow().clone(),
            String::from("Hello world!")
        );

        // `Rc` of o1 is alive.
        assert_eq!(Rc::strong_count(o1.as_ref().unwrap()), 2);
        let o3 = o1.take();
        o1 = o3;
        assert_eq!(Rc::strong_count(o1.as_ref().unwrap()), 2);

        // drop o1
        assert_eq!(Rc::strong_count(o1.as_ref().unwrap()), 2);
        o1 = None;
        assert!(o1.is_none());
        assert_eq!(Rc::strong_count(o2.as_ref().unwrap()), 1);
    }

    #[test]
    fn test_option_take() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);
        o1.as_ref().map(|s| assert_eq!(s, &String::from("Hello")));

        let o2 = o1.take();  // o1 is not moved.
        assert_eq!(o1, None);
        assert_eq!(o1.is_some(), false);
        o2.as_ref().map(|s| assert_eq!(s, &String::from("Hello")));

        let mut s2 = o2.unwrap();
        assert_eq!(s2, String::from("Hello"));
        s2.push_str(" world");

        let o1: Option<String> = Some(s2);
        assert_eq!(o1.unwrap(), String::from("Hello world"));
    }
}
