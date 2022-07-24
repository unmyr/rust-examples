use std::result::Result;
use std::fmt::{Formatter, Error};
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Xor { E, G }
impl std::fmt::Display for Xor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Xor::E => write!(f, "0"),
            Xor::G => write!(f, "1"),
        }
    }
}

fn xor_op(x: &Xor, y: &Xor) -> Xor {
    match (x, y) {
        (Xor::E, Xor::G) | (Xor::G, Xor::E) => Xor::G,
        _ => Xor::E
    }
}

pub fn xor_inv(x_ref: &Xor) -> Xor {
    let v = vec![Xor::E, Xor::G];
    let result = v.into_iter().filter(
        |y_ref| xor_op(x_ref, y_ref) == Xor::E
    ).collect::<Vec<_>>().pop();
    result.unwrap()
}

pub fn xor_calc(mut v: VecDeque<Xor>) -> VecDeque<Xor> {
    while v.len() >= 2 {
        let x = v.pop_front().unwrap();
        let y = v.pop_front().unwrap();
        v.push_front(xor_op(&x, &y));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_inv() {
        assert_eq!(xor_inv(&Xor::G), Xor::G);
        assert_eq!(xor_inv(&Xor::E), Xor::E);
    }

    #[test]
    fn test_xor_calc() {
        assert_eq!(xor_calc(VecDeque::from([Xor::E])), vec![Xor::E]);
        assert_eq!(xor_calc(VecDeque::from([Xor::G])), vec![Xor::G]);
        assert_eq!(xor_calc(VecDeque::from([Xor::E, Xor::E])), vec![Xor::E]);
        assert_eq!(xor_calc(VecDeque::from([Xor::E, Xor::G])), vec![Xor::G]);
        assert_eq!(xor_calc(VecDeque::from([Xor::G, Xor::E])), vec![Xor::G]);
        assert_eq!(xor_calc(VecDeque::from([Xor::G, Xor::G])), vec![Xor::E]);

        assert_eq!(
            xor_calc(VecDeque::from([Xor::G, Xor::G, Xor::G])),
            vec![Xor::G]
        );
        assert_eq!(
            xor_calc(VecDeque::from([Xor::G, Xor::G, Xor::G, Xor::G])),
            vec![Xor::E]
        );
    }
}
