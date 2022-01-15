// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
/// # Examples
///
/// ```
/// use unwrap::give_adult;
/// let result = give_adult(Some("water"));
/// assert_eq!(result, "water? How nice.");
/// ```
pub fn give_adult(drink: Option<&str>) -> String {
    // Specify a course of action for each case.
    let message = match drink {
        Some("lemonade") => String::from("Yuck! Too sugary."),
        Some(inner)   => String::from(format!("{}? How nice.", inner)),
        None          => String::from("No drink? Oh well."),
    };
    return message;
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
/// # Examples
///
/// ```
/// use unwrap::drink;
/// let result = drink(Some("water"));
/// assert_eq!(result, "I love water!!!!!");
/// ```
///
/// ```should_panic
/// use unwrap::drink;
/// drink(Some("lemonade"));
/// ```
pub fn drink(drink: Option<&str>) -> String {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    return String::from(format!("I love {}!!!!!", inside));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_normal() {
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let water  = Some("water");
        let lemonade = Some("lemonade");
        let void  = None;

        assert_eq!(give_adult(water), "water? How nice.");
        assert_eq!(give_adult(lemonade), "Yuck! Too sugary.");
        assert_eq!(give_adult(void), "No drink? Oh well.");

        let coffee = Some("coffee");
        assert_eq!(drink(water), "I love water!!!!!");
        assert_eq!(drink(coffee), "I love coffee!!!!!");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap_none() {
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let nothing = None;
        drink(nothing);
    }

    #[test]
    #[should_panic(expected = "AAAaaaaa!!!!")]
    fn test_unwrap_panic() {
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let lemonade = Some("lemonade");
        drink(lemonade);
    }
}
