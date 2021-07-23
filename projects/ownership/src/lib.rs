#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn append_world(some_string: &mut String) {
    some_string.push_str(", world");
}

#[allow(dead_code)]
fn get_hello() -> String {
    let hello = String::from("hello");
    hello
}

#[allow(dead_code)]
fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return i;
        }
    }

    s.len()
}

#[allow(dead_code)]
fn get_first_word_with_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
fn get_first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_string() {
        let hello = String::from("hello");
        let len = calculate_length(&hello);
        assert_eq!(len, 5);
        assert_eq!(hello, "hello");
    }

    #[test]
    fn test_mutable_string() {
        let mut hello = String::from("hello");
        append_world(&mut hello);
        assert_eq!(hello, "hello, world");
    }

    #[test]
    fn test_move_owner_string() {
        let mut hello = get_hello();
        assert_eq!(hello, "hello");
        append_world(&mut hello);
        assert_eq!(hello, "hello, world");
    }

    #[test]
    fn test_immutable_slice() {
        let hello_world = String::from("hello, world");
        let len = first_word_len(&hello_world);
        assert_eq!(len, 5);
        assert_eq!(hello_world, "hello, world");
        let hello_slice1 = get_first_word_with_string(&hello_world);
        assert_eq!(hello_slice1, "hello");
        let hello_slice1 = get_first_word_with_slice(&hello_world[..]);
        assert_eq!(hello_slice1, "hello");
    }
}
