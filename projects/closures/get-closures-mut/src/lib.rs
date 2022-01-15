// Rust 1.26 later.
/// # Examples
///
/// ```
/// use get_closures_mut::get_push_char_move;
///
/// let initial_str = String::from("hello");
/// let str_to_append = String::from("!");
/// let mut push_em = get_push_char_move(initial_str, str_to_append);
///
/// let new_str = push_em();
/// assert_eq!(new_str, "hello!");
/// ```
pub fn get_push_char_move(
    initial_str: String, str_to_append: String
) -> impl FnMut() -> String {
    let mut s = initial_str;
    let a = str_to_append;
    move || {
        s.push_str(&a);
        s.clone()
    }
}

/// # Examples
///
/// ```
/// use get_closures_mut::get_push_char_borrow;
///
/// let mut push_em = get_push_char_borrow("hello", "!");
/// let count = push_em();
/// assert_eq!(count, 6);
/// ```
pub fn get_push_char_borrow<'a>(
    initial_str: &str, str_to_append: &'a str
) -> impl 'a + FnMut() -> usize {
    let mut s = String::from(initial_str);
    move || {
        s.push_str(str_to_append);
        s.len()
    }
}

/// # Examples
///
/// ```
/// use get_closures_mut::get_counter_old_style;
///
/// let mut inc = get_counter_old_style(3);
/// let count = inc();
/// assert_eq!(count, 4);
/// ```
pub fn get_counter_old_style(initial_value: i32) -> Box<dyn FnMut() -> i32> {
    let mut x = initial_value;
    Box::new(
        move || {
            x += 1;
            x
        }
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_fnmut_closure() {
        // Rust 1.26 later.
        use crate::get_push_char_move as get_push_char;

        let initial_str = String::from("hello");
        let mut push_em = get_push_char(initial_str, String::from("!"));

        let new_str = push_em();
        assert_eq!(new_str, "hello!");

        let new_str = push_em();
        assert_eq!(new_str, "hello!!");
    }

    #[test]
    fn test_get_push_char_borrow() {
        // Rust 1.26 later.
        use crate::get_push_char_borrow as get_push_char;

        let initial_str = String::from("hello");
        let mut push_em = get_push_char(&initial_str, "!");
        let count = push_em();
        assert_eq!(count, 6);
        assert_eq!(initial_str, "hello");

        let count = push_em();
        assert_eq!(count, 7);
        assert_eq!(initial_str, "hello");
    }

    #[test]
    fn test_get_fnmut_closure_old_style() {
        use crate::get_counter_old_style;

        let mut inc = get_counter_old_style(3);
        let _count = inc();
        let count = inc();
        assert_eq!(count, 5);
    }

}
