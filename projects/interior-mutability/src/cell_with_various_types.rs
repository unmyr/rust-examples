#[cfg(test)]
mod tests {
    use std::cell::Cell; 

    #[test]
    fn test_update_some_fields() {
        let a_cell: Cell<(u8,&str)> = Cell::new((1, "foo"));

        let mut a_value = a_cell.take();
        a_value.1 = "bar";
        a_cell.replace(a_value);

        assert_eq!(a_cell.get(), (1, "bar"));
    }

    #[test]
    fn test_update_option() {
        let some_a_cell: Option<Cell<(u8,&str)>> = Some(Cell::new((1, "foo")));

        if let Some(ref a_cell) = some_a_cell {
            let mut a_value = a_cell.take();
            a_value.1 = "bar";
            a_cell.replace(a_value);
        }

        assert_eq!(some_a_cell.unwrap().get(), (1, "bar"));
    }

    #[test]
    fn test_string_within_cell() {
        let a_cell: Cell<(u8,String)> = Cell::new(
            (1, String::from("Alice"))
        );

        let mut a_value = a_cell.take();
        a_value.1 = String::from("Bob");
        a_cell.replace(a_value);

        // the method `get` exists for struct `std::cell::Cell<(u8, std::string::String)>`,
        // but its trait bounds were not satisfied
        //
        // note: the following trait bounds were not satisfied:
        //       `(u8, std::string::String): std::marker::Copy`
        // assert_eq!(a_cell.get(), (1, String::from("Bob")));
    }

    #[test]
    fn test_slice_basic() {
        let names: Vec<Cell<&str>> = vec![
            Cell::new("Alice"),
            Cell::new("Bob"),
            Cell::new("Carol"),
        ];

        let slice_a: &[Cell<&str>] = &names[0..=1];
        let slice_b: &[Cell<&str>] = &names[1..=2];

        slice_b[0].replace("Billy");

        assert_eq!(slice_a[1].get(), "Billy");
        assert_eq!(names[1].get(), "Billy");
    }

    #[test]
    fn test_slice_and_from_mut() {
        let slice: &mut [&str] = &mut ["Alice", "Bob", "Carol"];
        let cell_slice: &Cell<[&str]> = Cell::from_mut(slice);
        let names: &[Cell<&str>] = cell_slice.as_slice_of_cells();

        let slice_a = &names[0..=1];
        let slice_b = &names[1..=2];

        slice_b[0].replace("Billy");

        assert_eq!(slice_a[1].get(), "Billy");
        assert_eq!(names[1].get(), "Billy");
    }
}
