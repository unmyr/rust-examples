// If it is an even number, divide by 2.
/// # Examples
///
/// ```
/// use result::halves_if_even;
/// halves_if_even(2);
/// halves_if_even(3).err().expect("Not even.");
/// ```
pub fn halves_if_even(i: i32) -> Result<i32, &'static str> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err(&"Not even.")
    }
}

// If it is an even number, divide it by 2,
// and if the number is an even number, add 1.
/// # Examples
///
/// ```
/// use result::halves_number_to_odd;
/// halves_number_to_odd(2);
/// halves_number_to_odd(3).err().expect("Not even.");
/// ```
pub fn halves_number_to_odd(i: i32) -> Result<i32, &'static str> {
    let mut result = halves_if_even(i)?;
    if (result % 2) == 0 {
        result += 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_results_array() {
        let results = [Ok(100), Err("oops!")];
        for (i, r) in results.iter().enumerate() {
            let value = match r {
                Ok(v) => v / 2,
                Err(_) => -1,
            };

            assert_eq!(value, if i == 0 { 50 } else { -1 });
        }
    }

    #[test]
    fn test_results_tuple() {
        let result_tuple: Result<(usize, &str), &str> = Ok((2, &"Hello"));
        let value = match result_tuple {
            Ok(tuple) => format!("usize={} str=\"{}\"", tuple.0, tuple.1),
            Err(error) => format!("{}", error)
        };
        assert_eq!(value, "usize=2 str=\"Hello\"");

        let result_tuple: Result<(usize, &str), &str> = Ok((2, &"World"));
        let value = match result_tuple {
            Ok((x, y)) => format!("usize={} str=\"{}\"", x, y),
            Err(error) => format!("{}", error)
        };
        assert_eq!(value, "usize=2 str=\"World\"");
    }

    #[test]
    fn test_question_mark_operator() {
        use crate::halves_number_to_odd;

        // 4/2 + 1 = 3
        let value = halves_number_to_odd(4);
        assert_eq!(value, Ok(3));

        let value = halves_number_to_odd(3);
        assert_eq!(value, Err("Not even."));
    }

    #[test]
    fn test_and_then() {
        use crate::halves_if_even;
        let value = halves_if_even(4).and_then(halves_if_even);
        assert_eq!(value, Ok(1));

        let value = halves_if_even(2).and_then(halves_if_even);
        assert_eq!(value, Err("Not even."));
    }

    #[test]
    #[should_panic(expected = "None detected!")]
    fn test_unwrap_or_else_simple() {
        (None as Option<u8>).unwrap_or_else(
            || panic!("None detected!")
        );
    }

    #[test]
    #[should_panic(expected = "strong_count grater than 1 : st=2")]
    fn test_unwrap_or_else() {
        use std::rc::Rc;
        use std::cell::RefCell;

        let a: Rc<Option<RefCell<String>>>;
        let b: Rc<Option<RefCell<String>>>;
        a = Rc::new(Some(RefCell::new(String::from("a"))));
        b = Rc::clone(&a);
        assert!(&a == &b);

        let st_a = Rc::strong_count(&a);
        let result: Option<RefCell<String>>;
        result = Rc::try_unwrap(a).ok().unwrap_or_else(
            || panic!("strong_count grater than 1 : st={}", st_a)
        );
        assert_eq!(String::from("a"), result.unwrap().borrow().clone());
    }
}
