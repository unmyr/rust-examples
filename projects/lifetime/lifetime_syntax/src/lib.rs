#[allow(dead_code)]
fn get_longest_slice<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_with_fn_same() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = get_longest_slice(string1.as_str(), string2);
        assert_eq!(result, "abcd");
    }

    #[test]
    fn test_lifetime_with_fn_different_block() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = get_longest_slice(string1.as_str(), string2.as_str());
            assert_eq!(result, "long string is long");
        }
        // NG: compile error.
        //    `string2` does not live long enough
        //    borrowed value does not live long enough
        // assert_eq!(result, "long string is long");
    }

    #[test]
    fn test_lifetime_with_fn_different_static() {
        let string1 = String::from("abc");
        let result;
        {
            let string2: &'static str = "I have a static lifetime.";
            result = get_longest_slice(string1.as_str(), string2);
        }
        assert_eq!(result, "I have a static lifetime.");
    }

    #[test]
    fn test_lifetime_with_struct() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
        assert_eq!(i.part, "Call me Ishmael");
    }
}
