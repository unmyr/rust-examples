#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_imm() {
        let immut_val = String::from("immut");
        let fn_closure = || immut_val.len();
        assert_eq!(fn_closure(), 5);
    }
}