pub enum Message {
    Hello {id: i32}
}

/// # Examples
///
/// ```
/// use match_expressions::range::{Message, get_message_str};
/// get_message_str(&(Message::Hello { id: 5 }));
/// ```
pub fn get_message_str(msg: &Message) -> String {
    let message = match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => format!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            format!("Found an id in another range")
        }
        Message::Hello { id } => format!("Found some other id: {}", id),
    };
    message
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_message_str() {
        use crate::range::{Message, get_message_str};

        let msg = Message::Hello { id: 5 };
        assert_eq!(get_message_str(&msg), "Found an id in range: 5");

        let msg = Message::Hello { id: 12 };
        assert_eq!(get_message_str(&msg), "Found an id in another range");

        let msg = Message::Hello { id: 1 };
        assert_eq!(get_message_str(&msg), "Found some other id: 1");
    }
}