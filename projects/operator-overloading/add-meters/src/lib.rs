use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        use crate::{Meters, Millimeters};
        assert_eq!(
            Millimeters(1) + Meters(2),
            Millimeters(2001)
        )
    }
}
