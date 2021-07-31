
pub struct Person {
    #[allow(dead_code)]
    first_name: String,
    #[allow(dead_code)]
    last_name: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_joint_capturing() {
        use crate::Person;
        let alice_wonderland = Person {
            first_name: String::from("Alice"),
            last_name: String::from("Wonder"),
        };
        let format_first_name = || format!("First name: {}", alice_wonderland.first_name);
        // alice_wonderland.last_name.push_str("land");  // mutable borrow occurs here.
        assert_eq!(format_first_name(), "First name: Alice");
    }
}
