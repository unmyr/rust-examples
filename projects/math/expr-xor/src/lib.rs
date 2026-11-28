use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

#[derive(Clone)]
pub enum XorExpr {
    Zero,
    One,
    Variable(String),
    Xor(Box<XorExpr>, Box<XorExpr>),
}

impl Add for XorExpr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        xor(self, rhs)
    }
}

fn xor(lhs: XorExpr, rhs: XorExpr) -> XorExpr {
    match (lhs, rhs) {
        // Zero is identity for XOR
        (lhs, XorExpr::Zero) | (XorExpr::Zero, lhs) => lhs,
        // One is the negation for XOR
        (XorExpr::One, XorExpr::One) => XorExpr::Zero,
        (XorExpr::One, XorExpr::Variable(rhs)) => {
            XorExpr::Xor(Box::new(XorExpr::One), Box::new(XorExpr::Variable(rhs)))
        }
        (XorExpr::Variable(lhs), XorExpr::One) => {
            XorExpr::Xor(Box::new(XorExpr::Variable(lhs)), Box::new(XorExpr::One))
        }
        (XorExpr::Variable(lhs), XorExpr::Variable(rhs)) => {
            if lhs == rhs {
                XorExpr::Zero
            } else {
                XorExpr::Xor(
                    Box::new(XorExpr::Variable(lhs)),
                    Box::new(XorExpr::Variable(rhs)),
                )
            }
        }
        (lhs, rhs) => XorExpr::Xor(Box::new(lhs), Box::new(rhs)),
    }
}

pub fn simplify(expr: &XorExpr) -> XorExpr {
    fn collect_terms(
        expr: &XorExpr,
        const_terms: &mut HashMap<&u8, usize>,
        variable_terms: &mut Vec<String>,
    ) {
        match expr {
            XorExpr::Zero => {}
            XorExpr::One => {
                let count = const_terms.entry(&1).or_insert(0);
                *count += 1;
            }
            XorExpr::Variable(variable_name) => {
                if let Some(position) = variable_terms.iter().position(|key| *key == *variable_name)
                {
                    variable_terms.remove(position);
                } else {
                    variable_terms.push(variable_name.to_string())
                }
            }
            XorExpr::Xor(left, right) => {
                collect_terms(left, const_terms, variable_terms);
                collect_terms(right, const_terms, variable_terms);
            }
        }
    }

    let mut const_terms = HashMap::new();
    let mut variable_terms = Vec::new();
    collect_terms(expr, &mut const_terms, &mut variable_terms);
    let one_count = match const_terms.get(&1) {
        Some(count) => *count,
        None => 0,
    };

    let expr = if one_count % 2 == 1 {
        XorExpr::One
    } else {
        XorExpr::Zero
    };
    variable_terms.into_iter().fold(expr, |left, variable| xor(left, XorExpr::Variable(variable)))
}

impl fmt::Display for XorExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XorExpr::Zero => write!(f, "0"),
            XorExpr::One => write!(f, "1"),
            XorExpr::Variable(s) => write!(f, "{}", s),
            XorExpr::Xor(left, right) => write!(f, "({} ⊕ {})", left, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_constants() {
        assert_eq!((XorExpr::Zero + XorExpr::Zero).to_string(), "0");
        assert_eq!((XorExpr::Zero + XorExpr::One).to_string(), "1");
        assert_eq!((XorExpr::One + XorExpr::Zero).to_string(), "1");
        assert_eq!((XorExpr::One + XorExpr::One).to_string(), "0");
        assert_eq!(
            (XorExpr::One + XorExpr::Zero + XorExpr::One).to_string(),
            "0"
        );
        assert_eq!(simplify(&XorExpr::Zero).to_string(), "0");
        assert_eq!(simplify(&XorExpr::One).to_string(), "1");
    }

    #[test]
    fn test_xor_variables() {
        let x = XorExpr::Variable("x".to_string());
        let y = XorExpr::Variable("y".to_string());
        // x ⊕ y
        let expr = x.clone() + y.clone();
        assert_eq!(expr.to_string(), "(x ⊕ y)");
        // x ⊕ x → 0
        let expr = x.clone() + x.clone();
        assert_eq!(expr.to_string(), "0");
        // x ⊕ y ⊕ x ⊕ y → 0 ⊕ 0 → 0
        let expr = x.clone() + y.clone() + x.clone() + y.clone();
        assert_eq!(expr.to_string(), "(((x ⊕ y) ⊕ x) ⊕ y)");
        assert_eq!(simplify(&expr).to_string(), "0");
    }

    #[test]
    fn test_xor_mixed_operands() {
        let x = XorExpr::Variable("x".to_string());
        let y = XorExpr::Variable("y".to_string());
        let expr = XorExpr::One + x.clone();
        assert_eq!(expr.to_string(), "(1 ⊕ x)");
        // x ⊕ x ⊕ x → 0 ⊕ x → x
        let expr = x.clone() + x.clone() + x.clone();
        assert_eq!(expr.to_string(), "x");
        assert_eq!(simplify(&expr).to_string(), "x");

        // Add builds a nested tree; simplify flattens it and cancels repeated terms.
        // 1 ⊕ x ⊕ x ⊕ x → 1 ⊕ 0 ⊕ x → 1 ⊕ x
        let expr = XorExpr::One + x.clone() + x.clone() + x.clone();
        assert_eq!(expr.to_string(), "(((1 ⊕ x) ⊕ x) ⊕ x)");
        assert_eq!(simplify(&expr).to_string(), "(1 ⊕ x)");
        // 1 ⊕ x ⊕ x → 1 ⊕ 0 → 1
        let expr = XorExpr::One + x.clone() + x.clone();
        assert_eq!(simplify(&expr).to_string(), "1");
        // 1 ⊕ x ⊕ y
        let expr = XorExpr::One + x.clone() + y.clone();
        assert_eq!(expr.to_string(), "((1 ⊕ x) ⊕ y)");
        assert_eq!(simplify(&expr).to_string(), "((1 ⊕ x) ⊕ y)");
        // x ⊕ y ⊕ x
        let expr = x.clone() + y.clone() + x.clone();
        assert_eq!(expr.to_string(), "((x ⊕ y) ⊕ x)");
        assert_eq!(simplify(&expr).to_string(), "y");
    }
}
