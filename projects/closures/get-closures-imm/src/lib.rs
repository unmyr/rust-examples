// Rust 1.26 later.
/// # Examples
/// 
/// ```
/// use get_closures_imm::get_strlen_move;
/// 
/// let strlen = get_strlen_move(String::from("immut"));
/// let len = strlen();
/// assert_eq!(len, 5);
/// ```
pub fn get_strlen_move(s: String) -> impl Fn() -> usize {
    let val_moved = s;
    move || val_moved.len()
}

// Rust 1.26 later.
/// # Examples
/// 
/// ```
/// use get_closures_imm::get_strlen_borrow;
/// 
/// let hello = String::from("hello");
/// let strlen = get_strlen_borrow(&hello);
/// let len = strlen();
/// assert_eq!(len, 5);
/// ```
pub fn get_strlen_borrow<'a>(s: &'a str) -> impl 'a + Fn() -> usize {
    move || s.len()
}

/// # Examples
/// 
/// ```
/// use get_closures_imm::get_strlen_old_style_move;
/// 
/// let hello = String::from("hello");
/// let strlen = get_strlen_old_style_move(hello);
/// let len = strlen();
/// assert_eq!(len, 5);
/// ```
pub fn get_strlen_old_style_move(s: String) -> Box<dyn Fn() -> usize> {
    let val_moved = s;

    Box::new(
        move || val_moved.len()
    )
}

pub fn get_strlen_old_style_borrow<'a>(s: &'a str) -> Box<dyn Fn() -> usize + 'a> {
    Box::new(move || s.len())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_fn_closure_move() {
        // Rust 1.26 later.
        use crate::get_strlen_move;

        let str_hello = String::from("hello");
        let str_hello_john = String::from("Hello John!");
        let hello_len = get_strlen_move(str_hello);
        let hello_john_len = get_strlen_move(str_hello_john);

        let len = hello_len();
        assert_eq!(len, 5);

        let len = hello_john_len();
        assert_eq!(len, 11);

        let len = hello_len();
        assert_eq!(len, 5);

        // assert_eq!(str_hello, String::from("hello"));
        // assert_eq!(str_hello_john, String::from("Hello John!"));
    }

    #[test]
    fn test_get_strlen_borrow() {
        // Rust 1.26 later.
        use crate::get_strlen_borrow;

        let str_hello = String::from("hello");
        let str_hello_john = String::from("Hello John!");
        let hello_len = get_strlen_borrow(&str_hello);
        let hello_john_len = get_strlen_borrow(&str_hello_john);

        let len = hello_len();
        assert_eq!(len, 5);

        let len = hello_john_len();
        assert_eq!(len, 11);

        let len = hello_len();
        assert_eq!(len, 5);

        assert_eq!(str_hello, String::from("hello"));
        assert_eq!(str_hello_john, String::from("Hello John!"));
    }

    #[test]
    fn test_get_fn_closure_old_style() {
        use crate::get_strlen_old_style_move;

        let hello_len = get_strlen_old_style_move(String::from("hello"));
        let hello_john_len = get_strlen_old_style_move(String::from("Hello John!"));

        let len = hello_len();
        assert_eq!(len, 5);

        let len = hello_john_len();
        assert_eq!(len, 11);

        let len = hello_len();
        assert_eq!(len, 5);
    }

    #[test]
    fn test_get_fn_closure_old_style_borrow() {
        use crate::get_strlen_old_style_borrow;

        let str_hello = String::from("hello");
        let str_hello_john = String::from("Hello John!");
        let hello_len = get_strlen_old_style_borrow(&str_hello);
        let hello_john_len = get_strlen_old_style_borrow(&str_hello_john);

        let len = hello_len();
        assert_eq!(len, 5);

        let len = hello_john_len();
        assert_eq!(len, 11);

        let len = hello_len();
        assert_eq!(len, 5);

        assert_eq!(str_hello, String::from("hello"));
        assert_eq!(str_hello_john, String::from("Hello John!"));
    }
}
