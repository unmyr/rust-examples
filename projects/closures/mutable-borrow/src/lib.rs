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
}