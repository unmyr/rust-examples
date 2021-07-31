pub fn call_equal_to_x_fn<F>(f: F, val: u32) -> bool where F: Fn(u32) -> bool {
    f(val)
}

pub fn call_equal_to_x_fn_mut<F>(mut f: F, val: u32) -> bool where F: FnMut(u32) -> bool {
    f(val)
}

pub fn call_equal_to_x_fn_once<F>(f: F, val: u32) -> bool where F: FnOnce(u32) -> bool {
    f(val)
}

pub fn call_no_arg_fn<F>(f: F) where F: Fn() {
    f()
}

pub fn call_no_arg_fn_mut<F>(mut f: F) where F: FnMut() {
    f();
    f()
}

pub fn call_no_arg_fn_once<F>(f: F) where F: FnOnce() {
    f();
    // f()  // NG: value used here after move.
}


pub fn call_and_ret_string_fn<F>(f: F) -> String
where F: Fn() -> String {
    f()
}

pub fn call_and_ret_string_fn_mut<F>(mut f: F) -> String
where F: FnMut() -> String {
    f()
}

pub fn call_and_ret_string_fn_once<F>(f: F) -> String
where F: FnOnce() -> String {
    f()
    // f(); f()  // NG: value used here after move.
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_imm() {
        let x: u32 = 5;
        let equal_to_x = |y| x == y;

        assert_eq!(equal_to_x(4), false);
        assert_eq!(equal_to_x(5), true);
    }

    #[test]
    fn test_fn_mut() {
        let mut x: u32 = 5;
        let mut inc = || x += 1;

        inc();
        inc();
        assert_eq!(x, 7);
    }

    #[test]
    fn test_fn_mut_move() {
        let hello1 = String::from("Hello World");
        let push_mark = move |mut s: String| {s.push('!'); s};

        let hello2 = push_mark(hello1);  // `hello1` is moved.
        assert_eq!(hello2, "Hello World!");
        let hello3 = push_mark(hello2);  // `hello2` is moved.
        assert_eq!(hello3, "Hello World!!");
    }

    #[test]
    fn test_fn_once() {
        let s = String::from("Hello");
        let consume_and_return_s = || s;

        let result = consume_and_return_s();  // s is moved.

        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_call_fn_imm() {
        use crate::call_equal_to_x_fn;
        use crate::call_equal_to_x_fn_mut;
        use crate::call_equal_to_x_fn_once;
        let x: u32 = 5;
        let equal_to_x = |y| x == y;

        assert_eq!(call_equal_to_x_fn(equal_to_x, 4), false);
        assert_eq!(call_equal_to_x_fn(equal_to_x, 5), true);

        assert_eq!(call_equal_to_x_fn_mut(equal_to_x, 4), false);
        assert_eq!(call_equal_to_x_fn_mut(equal_to_x, 5), true);

        assert_eq!(call_equal_to_x_fn_once(equal_to_x, 4), false);
        assert_eq!(call_equal_to_x_fn_once(equal_to_x, 5), true);
    }

    #[test]
    fn test_call_fn_mut() {
        #[allow(unused_imports)]
        use crate::call_no_arg_fn;
        use crate::call_no_arg_fn_mut;
        use crate::call_no_arg_fn_once;

        {
            let mut x: u32 = 5;
            let inc = || x += 1;
    
            call_no_arg_fn_mut(inc);

            assert_eq!(x, 7);
        }

        {
            let mut x: u32 = 5;
            let inc = || x += 1;
    
            call_no_arg_fn_once(inc);

            assert_eq!(x, 6);
        }
    }


    #[test]
    fn test_call_fn_once() {
        #[allow(unused_imports)]
        use crate::call_and_ret_string_fn;
        #[allow(unused_imports)]
        use crate::call_and_ret_string_fn_mut;
        use crate::call_and_ret_string_fn_once;

        {
            let s = String::from("Hello");
            let consume_and_return_s = || s;
    
            let result = call_and_ret_string_fn_once(consume_and_return_s);

            assert_eq!(result, "Hello");
        }
    }
}
