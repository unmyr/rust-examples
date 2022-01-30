#[cfg(test)]
mod tests {
    #[test]
    fn lower_to_upper() {
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = lowercase.chars().map(
            |c| c.to_uppercase().next().unwrap()
        ).collect::<Vec<_>>();

        assert_eq!(
            uppercase.iter().collect::<String>(),
            String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    fn test_generate_alphabet() {
        // Generate the lowercase and uppercase English alphabet.
        let alphabet = (b'A'..=b'z')       // Start as u8
            .map(|c| c as char)            // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>();          // Collect as Vec<char>
        assert_eq!(
            alphabet.iter().collect::<String>(),
            String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
        );
    }
}
