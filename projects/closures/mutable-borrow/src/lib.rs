pub fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f()
}

pub fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f(); f()
}

pub fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// Rust 1.26 later.
/// # Examples
/// 
/// ```
/// use mutable_borrow::get_counter;
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

// Rust 1.26 later.
/// # Examples
/// 
/// ```
/// use mutable_borrow::get_counter_old_style;
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
    fn test_fn_mut() {
        let mut mut_val = String::from("mut");
        let mut fnmut_closure = || {
            mut_val.push_str("-new");
        };
        fnmut_closure();
        assert_eq!(mut_val, "mut-new");
    }

    #[test]
    fn test_call_fn_mut() {
        use crate::call_fn_mut;

        let mut mut_val = String::from("mut");
        let fnmut_closure = || {
            mut_val.push_str("-new");
        };

        call_fn_mut(fnmut_closure);
        assert_eq!(mut_val, "mut-new-new");
    }

    #[test]
    fn test_call_fn_once() {
        use crate::call_fn_once;

        let mut mut_val = String::from("mut");
        let fnmut_closure = || {
            mut_val.push_str("-new");
        };

        call_fn_once(fnmut_closure);
        assert_eq!(mut_val, "mut-new");
    }

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