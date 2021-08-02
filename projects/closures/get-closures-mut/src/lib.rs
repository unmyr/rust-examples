// Rust 1.26 later.
/// # Examples
/// 
/// ```
/// use get_closures_mut::get_counter;
/// 
/// let mut inc = get_counter(3);
/// let count = inc();
/// assert_eq!(count, 4);
/// ```
pub fn get_counter(initial_value: i32) -> impl FnMut() -> i32 {
    let mut x = initial_value;
    move || {
        x += 1;
        x
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
        use crate::get_counter;

        let mut inc = get_counter(3);
        let _count = inc();
        let count = inc();
        assert_eq!(count, 5);
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
