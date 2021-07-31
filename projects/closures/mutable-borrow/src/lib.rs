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
}