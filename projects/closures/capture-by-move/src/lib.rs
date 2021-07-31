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
    fn test_fn_once() {
        let mov_val = String::from("value");
        let fnonce_closure = || {
            #[allow(unused_variables)]
            let moved_value = mov_val;
        };
        fnonce_closure();
    }

    #[test]
    fn test_call_fn_mut() {
        use crate::call_fn_once;

        let mov_val = String::from("value");
        let fnonce_closure = || {
            #[allow(unused_variables)]
            let moved_value = mov_val;
        };

        call_fn_once(fnonce_closure);
    }
}