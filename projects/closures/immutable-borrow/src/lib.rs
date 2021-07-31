pub fn call_fn<F>(f: F) -> usize
where
    F: Fn() -> usize,
{
    f()
}

pub fn call_fn_mut<F>(mut f: F) -> usize
where
    F: FnMut() -> usize,
{
    f(); f()
}

pub fn call_fn_once<F>(f: F) -> usize
where
    F: FnOnce() -> usize,
{
    f()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_imm() {
        let immut_val = String::from("immut");
        let fn_closure = || immut_val.len();
        assert_eq!(fn_closure(), 5);
    }

    #[test]
    fn test_call_fn() {
        use crate::call_fn;

        let immut_val = String::from("immut");
        let fn_closure = || immut_val.len();

        assert_eq!(call_fn(fn_closure), 5);
    }


    #[test]
    fn test_call_fn_mut() {
        use crate::call_fn_mut;

        let immut_val = String::from("immut");
        let fn_closure = || immut_val.len();

        assert_eq!(call_fn_mut(fn_closure), 5);
        assert_eq!(call_fn_mut(fn_closure), 5);
    }


    #[test]
    fn test_call_fn_once() {
        use crate::call_fn_once;

        let immut_val = String::from("immut");
        let fn_closure = || immut_val.len();

        assert_eq!(call_fn_once(fn_closure), 5);
    }
}