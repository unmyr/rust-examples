use std::fmt;
use std::ops::{Add, Div, Mul, Neg};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Constant(i64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
    PowExpr(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Ln(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Neg(Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
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

    pub fn pow_expr(self, exponent: Expr) -> Self {
        power_expr(self, exponent)
    }

    pub fn exp(self) -> Self {
        Self::Exp(Box::new(self))
    }

    pub fn ln(self) -> Self {
        Self::Ln(Box::new(self))
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

impl Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        quotient(self, rhs)
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
        Expr::PowExpr(base, exponent) => {
            let base_derivative = differentiate(base, variable);
            let exponent_derivative = differentiate(exponent, variable);
            let base_derivative_term = base_derivative
                * (**exponent).clone()
                * power_expr((**base).clone(), (**exponent).clone() + Expr::constant(-1));
            let exponent_derivative_term =
                exponent_derivative * (**base).clone().ln() * expr.clone();

            base_derivative_term + exponent_derivative_term
        }
        Expr::Exp(inner) => (**inner).clone().exp() * differentiate(inner, variable),
        Expr::Ln(inner) => {
            differentiate(inner, variable) / (**inner).clone()
        }
        Expr::Sin(inner) => (**inner).clone().cos() * differentiate(inner, variable),
        Expr::Cos(inner) => -((**inner).clone().sin() * differentiate(inner, variable)),
        Expr::Neg(inner) => -differentiate(inner, variable),
        Expr::Div(numerator, denominator) => {
            (differentiate(numerator, variable) * (**denominator).clone()
                + -((**numerator).clone() * differentiate(denominator, variable)))
                / ((**denominator).clone() * (**denominator).clone())
        }
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

fn quotient(numerator: Expr, denominator: Expr) -> Expr {
    match (numerator, denominator) {
        (Expr::Constant(0), _) => Expr::constant(0),
        (numerator, Expr::Constant(1)) => numerator,
        (numerator, denominator) => Expr::Div(Box::new(numerator), Box::new(denominator)),
    }
}

fn power(base: Expr, exponent: u32) -> Expr {
    match exponent {
        0 => Expr::constant(1),
        1 => base,
        _ => Expr::Pow(Box::new(base), exponent),
    }
}

fn power_expr(base: Expr, exponent: Expr) -> Expr {
    match exponent {
        Expr::Constant(0) => Expr::constant(1),
        Expr::Constant(1) => base,
        exponent => Expr::PowExpr(Box::new(base), Box::new(exponent)),
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
            Self::Mul(_, _) | Self::Div(_, _) => 2,
            Self::Neg(_) => 3,
            Self::Pow(_, _) | Self::PowExpr(_, _) | Self::Exp(_) => 4,
            Self::Ln(_) => 5,
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
            Self::Div(numerator, denominator) => {
                numerator.fmt_with_precedence(f, precedence)?;
                f.write_str("/")?;
                denominator.fmt_with_precedence(f, precedence)?;
            }
            Self::Pow(base, exponent) => {
                base.fmt_with_precedence(f, precedence)?;
                write!(f, "^{exponent}")?;
            }
            Self::PowExpr(base, exponent) => {
                base.fmt_with_precedence(f, precedence)?;
                f.write_str("^")?;
                exponent.fmt_with_precedence(f, precedence)?;
            }
            Self::Exp(inner) => {
                f.write_str("e^")?;
                inner.fmt_with_precedence(f, precedence)?;
            }
            Self::Ln(inner) => {
                f.write_str("ln(")?;
                inner.fmt_with_precedence(f, 0)?;
                f.write_str(")")?;
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

    #[test]
    fn differentiates_x_times_e_to_the_x() {
        let x = Expr::variable("x");
        let expression = x.clone() * x.exp();

        let derivative = differentiate(&expression, "x");

        assert_eq!(derivative.to_string(), "e^x + x*e^x");
    }

    #[test]
    fn differentiates_a_to_the_x() {
        let a = Expr::variable("a");
        let x = Expr::variable("x");
        let expression = a.pow_expr(x);

        let derivative = differentiate(&expression, "x");

        assert_eq!(derivative.to_string(), "ln(a)*a^x");
    }

    #[test]
    fn differentiates_x_to_the_x_with_base_derivative() {
        let x = Expr::variable("x");
        let expression = x.clone().pow_expr(x.clone());

        let derivative = differentiate(&expression, "x");
        let exponent_minus_one = x.clone() + Expr::constant(-1);
        let base_derivative_term =
            x.clone() * x.clone().pow_expr(exponent_minus_one);
        let exponent_derivative_term =
            x.clone().ln() * expression;
        // Expected: x^x * ln(x) + x * x^(x-1)
        let expected = base_derivative_term.clone() + exponent_derivative_term;

        assert_eq!(derivative, expected);
        assert_ne!(base_derivative_term, Expr::constant(0));
    }
}
