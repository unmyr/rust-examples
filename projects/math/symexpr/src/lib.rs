use std::fmt;
use std::ops::{Add, Mul, Neg};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Constant(i64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    pub fn constant(value: i64) -> Self {
        Self::Constant(value)
    }

    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    pub fn pow(self, exponent: u32) -> Self {
        power(self, exponent)
    }

    pub fn sin(self) -> Self {
        Self::Sin(Box::new(self))
    }

    pub fn cos(self) -> Self {
        Self::Cos(Box::new(self))
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        sum(self, rhs)
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        product(self, rhs)
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        negate(self)
    }
}

pub fn differentiate(expr: &Expr, variable: &str) -> Expr {
    match expr {
        Expr::Constant(_) => Expr::constant(0),
        Expr::Variable(name) => Expr::constant(i64::from(name == variable)),
        Expr::Add(lhs, rhs) => differentiate(lhs, variable) + differentiate(rhs, variable),
        Expr::Mul(lhs, rhs) => {
            differentiate(lhs, variable) * (**rhs).clone()
                + (**lhs).clone() * differentiate(rhs, variable)
        }
        Expr::Pow(base, exponent) => {
            if *exponent == 0 {
                Expr::constant(0)
            } else {
                Expr::constant(i64::from(*exponent))
                    * power((**base).clone(), exponent - 1)
                    * differentiate(base, variable)
            }
        }
        Expr::Sin(inner) => (**inner).clone().cos() * differentiate(inner, variable),
        Expr::Cos(inner) => -((**inner).clone().sin() * differentiate(inner, variable)),
        Expr::Neg(inner) => -differentiate(inner, variable),
    }
}

fn sum(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (Expr::Constant(0), rhs) | (rhs, Expr::Constant(0)) => rhs,
        (Expr::Constant(lhs), Expr::Constant(rhs)) => Expr::constant(lhs + rhs),
        (lhs, rhs) => Expr::Add(Box::new(lhs), Box::new(rhs)),
    }
}

fn product(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (Expr::Constant(0), _) | (_, Expr::Constant(0)) => Expr::constant(0),
        (Expr::Constant(1), rhs) | (rhs, Expr::Constant(1)) => rhs,
        (Expr::Constant(lhs), Expr::Constant(rhs)) => Expr::constant(lhs * rhs),
        (lhs, rhs) => Expr::Mul(Box::new(lhs), Box::new(rhs)),
    }
}

fn power(base: Expr, exponent: u32) -> Expr {
    match exponent {
        0 => Expr::constant(1),
        1 => base,
        _ => Expr::Pow(Box::new(base), exponent),
    }
}

fn negate(expr: Expr) -> Expr {
    match expr {
        Expr::Constant(value) => Expr::constant(-value),
        Expr::Neg(inner) => *inner,
        expr => Expr::Neg(Box::new(expr)),
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_precedence(f, 0)
    }
}

impl Expr {
    fn fmt_with_precedence(
        &self,
        f: &mut fmt::Formatter<'_>,
        parent_precedence: u8,
    ) -> fmt::Result {
        let precedence = match self {
            Self::Add(_, _) => 1,
            Self::Mul(_, _) => 2,
            Self::Neg(_) => 3,
            Self::Pow(_, _) => 4,
            Self::Constant(_) | Self::Variable(_) | Self::Sin(_) | Self::Cos(_) => 5,
        };
        let needs_parentheses = precedence < parent_precedence;

        if needs_parentheses {
            f.write_str("(")?;
        }

        match self {
            Self::Constant(value) => write!(f, "{value}")?,
            Self::Variable(name) => f.write_str(name)?,
            Self::Add(lhs, rhs) => {
                lhs.fmt_with_precedence(f, precedence)?;
                f.write_str(" + ")?;
                rhs.fmt_with_precedence(f, precedence)?;
            }
            Self::Mul(lhs, rhs) => {
                lhs.fmt_with_precedence(f, precedence)?;
                f.write_str("*")?;
                rhs.fmt_with_precedence(f, precedence)?;
            }
            Self::Pow(base, exponent) => {
                base.fmt_with_precedence(f, precedence)?;
                write!(f, "^{exponent}")?;
            }
            Self::Sin(inner) => {
                f.write_str("sin(")?;
                inner.fmt_with_precedence(f, 0)?;
                f.write_str(")")?;
            }
            Self::Cos(inner) => {
                f.write_str("cos(")?;
                inner.fmt_with_precedence(f, 0)?;
                f.write_str(")")?;
            }
            Self::Neg(inner) => {
                f.write_str("-")?;
                inner.fmt_with_precedence(f, precedence)?;
            }
        }

        if needs_parentheses {
            f.write_str(")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differentiates_polynomial_plus_sine() {
        let x = Expr::variable("x");
        let expression = x.clone().pow(3) + x.sin();

        let derivative = differentiate(&expression, "x");

        assert_eq!(derivative.to_string(), "3*x^2 + cos(x)");
    }

    #[test]
    fn derivative_of_another_variable_is_zero() {
        let expression = Expr::variable("x").pow(3);

        assert_eq!(differentiate(&expression, "y"), Expr::constant(0));
    }
}
