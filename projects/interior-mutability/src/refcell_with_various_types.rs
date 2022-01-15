#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    fn test_update_some_fields() {
        let a_cell: RefCell<(u8,&str,String)> = RefCell::new(
            (1, "foo", String::from("Alice"))
        );

        a_cell.borrow_mut().1 = "bar";
        a_cell.borrow_mut().2.push_str("+");

        assert_eq!(
            a_cell.take(),
            (1, "bar", String::from("Alice+"))
        );
    }

    #[test]
    fn test_update_option() {
        let some_a_cell: Option<RefCell<(u8,&str)>> = Some(RefCell::new((1, "foo")));

        if let Some(ref a_cell) = some_a_cell {
            a_cell.borrow_mut().1 = "bar";
        }

        assert_eq!(some_a_cell.unwrap().take(), (1, "bar"));
    }

    #[test]
    fn test_slice_str_replace() {
        let names: Vec<RefCell<&str>> = vec![
            RefCell::new("Alice"),
            RefCell::new("Bob"),
            RefCell::new("Carol"),
        ];

        let slice_a: &[RefCell<&str>] = &names[0..=1];
        let slice_b: &[RefCell<&str>] = &names[1..=2];

        slice_b[0].replace("Billy");

        assert_eq!(slice_a[1].borrow().clone(), "Billy");
        assert_eq!(names[1].borrow().clone(), "Billy");
    }

    #[test]
    fn test_slice_string_update() {
        let names: Vec<RefCell<String>> = vec![
            RefCell::new(String::from("Alice")),
            RefCell::new(String::from("Bob")),
            RefCell::new(String::from("Carol")),
        ];

        let slice_a: &[RefCell<String>] = &names[0..=1];
        let slice_b: &[RefCell<String>] = &names[1..=2];

        slice_b[0].borrow_mut().push_str("+");

        assert_eq!(slice_a[1].borrow().clone(), "Bob+");
        assert_eq!(names[1].borrow().clone(), "Bob+");
    }
}
