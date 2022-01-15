/// # Examples
///
/// ```
/// use match_expressions::tuple::get_message_str;
/// assert_eq!(
///     get_message_str(Some("hello"), Some(3)),
///     "msg=hello, code=3"
/// );
/// ```
pub fn get_message_str(a: Option<&str>, b: Option<u8>) -> String {
    match (a, b) {
        (Some(msg), Some(code)) => {
            format!("msg={}, code={}", msg, code)
        },
        (_, None) => format!("msg=*, code=Nil"),
        (None, Some(code)) => format!("msg=Nil, code={}", code),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_message_str() {
        use crate::tuple::{get_message_str};

        assert_eq!(
            get_message_str(Some("hello"), Some(3)),
            "msg=hello, code=3"
        );
        assert_eq!(
            get_message_str(Some("hello"), None),
            "msg=*, code=Nil"
        );
        assert_eq!(
            get_message_str(None, Some(1)),
            "msg=Nil, code=1"
        );
        assert_eq!(
            get_message_str(None, None),
            "msg=*, code=Nil"
        );
    }
}