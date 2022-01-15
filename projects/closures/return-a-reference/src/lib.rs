pub struct Person {
    pub name : String,
    pub age : u8,
}

impl Person {
    /// # Examples
    ///
    /// ```
    /// use return_a_reference::{Person, person_get_name};
    ///
    /// let p = Person::new("Nobody", 24);
    /// ```
    #[allow(dead_code)]
    pub fn new(name: &str, age: u8) -> Person {
        Person {name: String::from(name), age: age}
    }
}

/// # Examples
///
/// ```
/// use return_a_reference::{Person, person_get_name};
///
/// let p = Person{ name: "Nobody".to_string(), age : 24};
///
/// assert_eq!(person_get_name(&p), "Nobody");
/// ```
pub fn person_get_name<'a>(person: &'a Person) -> &'a str {
    &person.name
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_return_reference_in_fn() {
        use crate::{Person, person_get_name};
        let p = Person::new("Nobody", 24);

        assert_eq!(person_get_name(&p), "Nobody");

        let name = |p| person_get_name(p);
        assert_eq!(name(&p), "Nobody");
    }

    #[test]
    fn test_return_reference_in_closure() {
        use crate::Person;
        let p = Person{ name: "Nobody".to_string(), age : 24};

        let age = |p : &Person| p.age;
        let name: for<'a> fn(&'a Person) -> &'a String = |p : &Person| &p.name;

        println! ("name={}, age={}" , name(&p), age(&p));
        assert_eq!(name(&p), &"Nobody");
        assert_eq!(age(&p), 24);
    }
}
