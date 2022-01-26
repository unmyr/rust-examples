#[cfg(test)]
mod tests {
    #[test]
    fn test_option() {
        let mut o1: Option<String> = Some(String::from("Hello"));
        assert_eq!(o1.is_some(), true);
        o1.as_ref().map(|s| assert_eq!(s, &String::from("Hello")));

        let o2 = o1.take();  // o1 is not moved.
        assert_eq!(o1, None);
        assert_eq!(o1.is_some(), false);
        o2.as_ref().map(|s| assert_eq!(s, &String::from("Hello")));

        let mut s2 = o2.unwrap();
        assert_eq!(s2, String::from("Hello"));
        s2.push_str(" world");

        let o1: Option<String> = Some(s2);
        assert_eq!(o1.unwrap(), String::from("Hello world"));
    }
}
