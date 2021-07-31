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
}