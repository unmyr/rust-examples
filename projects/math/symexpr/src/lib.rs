use std::collections::HashMap;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    VariableNotFound(String),
    DivisionByZero,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableNotFound(name) => write!(f, "no value provided for variable '{name}'"),
            Self::DivisionByZero => f.write_str("division by zero"),
        }
    }
}

impl std::error::Error for EvalError {}

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
        Expr::Ln(inner) => differentiate(inner, variable) / (**inner).clone(),
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

pub fn substitute(expr: &Expr, variables: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Constant(_) => expr.clone(),
        Expr::Variable(name) => variables.get(name).cloned().unwrap_or_else(|| expr.clone()),
        Expr::Add(lhs, rhs) => Expr::Add(
            Box::new(substitute(lhs, variables)),
            Box::new(substitute(rhs, variables)),
        ),
        Expr::Mul(lhs, rhs) => Expr::Mul(
            Box::new(substitute(lhs, variables)),
            Box::new(substitute(rhs, variables)),
        ),
        Expr::Pow(base, exponent) => Expr::Pow(Box::new(substitute(base, variables)), *exponent),
        Expr::PowExpr(base, exponent) => Expr::PowExpr(
            Box::new(substitute(base, variables)),
            Box::new(substitute(exponent, variables)),
        ),
        Expr::Exp(inner) => Expr::Exp(Box::new(substitute(inner, variables))),
        Expr::Ln(inner) => Expr::Ln(Box::new(substitute(inner, variables))),
        Expr::Sin(inner) => Expr::Sin(Box::new(substitute(inner, variables))),
        Expr::Cos(inner) => Expr::Cos(Box::new(substitute(inner, variables))),
        Expr::Neg(inner) => Expr::Neg(Box::new(substitute(inner, variables))),
        Expr::Div(numerator, denominator) => Expr::Div(
            Box::new(substitute(numerator, variables)),
            Box::new(substitute(denominator, variables)),
        ),
    }
}

pub fn evaluate(expr: &Expr, variables: &HashMap<String, f64>) -> Result<f64, EvalError> {
    match expr {
        Expr::Constant(value) => Ok(*value as f64),
        Expr::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| EvalError::VariableNotFound(name.clone())),
        Expr::Add(lhs, rhs) => Ok(evaluate(lhs, variables)? + evaluate(rhs, variables)?),
        Expr::Mul(lhs, rhs) => Ok(evaluate(lhs, variables)? * evaluate(rhs, variables)?),
        Expr::Pow(base, exponent) => Ok(integer_power(evaluate(base, variables)?, *exponent)),
        Expr::PowExpr(base, exponent) => {
            Ok(evaluate(base, variables)?.powf(evaluate(exponent, variables)?))
        }
        Expr::Exp(inner) => Ok(evaluate(inner, variables)?.exp()),
        Expr::Ln(inner) => Ok(evaluate(inner, variables)?.ln()),
        Expr::Sin(inner) => Ok(evaluate(inner, variables)?.sin()),
        Expr::Cos(inner) => Ok(evaluate(inner, variables)?.cos()),
        Expr::Neg(inner) => Ok(-evaluate(inner, variables)?),
        Expr::Div(numerator, denominator) => {
            let denominator = evaluate(denominator, variables)?;
            if denominator == 0.0 {
                return Err(EvalError::DivisionByZero);
            }
            Ok(evaluate(numerator, variables)? / denominator)
        }
    }
}

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Constant(_) | Expr::Variable(_) => expr.clone(),
        Expr::Add(lhs, rhs) => {
            let lhs = simplify(lhs);
            let rhs = simplify(rhs);
            if is_sin_squared_plus_cos_squared(&lhs, &rhs) {
                Expr::constant(1)
            } else {
                sum(lhs, rhs)
            }
        }
        Expr::Mul(lhs, rhs) => product(simplify(lhs), simplify(rhs)),
        Expr::Pow(base, exponent) => {
            let base = simplify(base);
            match (&base, exponent) {
                (Expr::Constant(value), exponent) => value
                    .checked_pow(*exponent)
                    .map(Expr::constant)
                    .unwrap_or_else(|| Expr::Pow(Box::new(base.clone()), *exponent)),
                (_, 0) => Expr::constant(1),
                (_, 1) => base,
                _ => Expr::Pow(Box::new(base), *exponent),
            }
        }
        Expr::PowExpr(base, exponent) => power_expr(simplify(base), simplify(exponent)),
        Expr::Exp(inner) => match simplify(inner) {
            Expr::Constant(0) => Expr::constant(1),
            inner => Expr::Exp(Box::new(inner)),
        },
        Expr::Ln(inner) => match simplify(inner) {
            Expr::Constant(1) => Expr::constant(0),
            inner => Expr::Ln(Box::new(inner)),
        },
        Expr::Sin(inner) => match simplify(inner) {
            Expr::Constant(0) => Expr::constant(0),
            inner => Expr::Sin(Box::new(inner)),
        },
        Expr::Cos(inner) => match simplify(inner) {
            Expr::Constant(0) => Expr::constant(1),
            inner => Expr::Cos(Box::new(inner)),
        },
        Expr::Neg(inner) => negate(simplify(inner)),
        Expr::Div(numerator, denominator) => quotient(simplify(numerator), simplify(denominator)),
    }
}

fn integer_power(mut base: f64, mut exponent: u32) -> f64 {
    let mut result = 1.0;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result *= base;
        }
        exponent >>= 1;
        if exponent > 0 {
            base *= base;
        }
    }
    result
}

fn is_sin_squared_plus_cos_squared(lhs: &Expr, rhs: &Expr) -> bool {
    fn square_argument(expr: &Expr) -> Option<(&Expr, bool)> {
        match expr {
            Expr::Pow(base, 2) => match base.as_ref() {
                Expr::Sin(inner) => Some((inner, true)),
                Expr::Cos(inner) => Some((inner, false)),
                _ => None,
            },
            Expr::PowExpr(base, exponent) if exponent.as_ref() == &Expr::Constant(2) => {
                match base.as_ref() {
                    Expr::Sin(inner) => Some((inner, true)),
                    Expr::Cos(inner) => Some((inner, false)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    match (square_argument(lhs), square_argument(rhs)) {
        (Some((lhs_arg, lhs_is_sin)), Some((rhs_arg, rhs_is_sin))) => {
            lhs_arg == rhs_arg && lhs_is_sin != rhs_is_sin
        }
        _ => false,
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
    match (&lhs, &rhs) {
        (base, Expr::PowExpr(pow_base, exponent))
            if base == pow_base.as_ref() && is_base_minus_one(base, exponent) =>
        {
            return power_expr(base.clone(), base.clone());
        }
        (Expr::PowExpr(pow_base, exponent), base)
            if base == pow_base.as_ref() && is_base_minus_one(base, exponent) =>
        {
            return power_expr(base.clone(), base.clone());
        }
        _ => {}
    }

    match (lhs, rhs) {
        (Expr::Constant(0), _) | (_, Expr::Constant(0)) => Expr::constant(0),
        (Expr::Constant(1), rhs) | (rhs, Expr::Constant(1)) => rhs,
        (Expr::Constant(lhs), Expr::Constant(rhs)) => Expr::constant(lhs * rhs),
        (lhs, rhs) => Expr::Mul(Box::new(lhs), Box::new(rhs)),
    }
}

fn is_base_minus_one(base: &Expr, exponent: &Expr) -> bool {
    matches!(
        exponent,
        Expr::Add(lhs, rhs)
            if (lhs.as_ref() == base && rhs.as_ref() == &Expr::Constant(-1))
                || (rhs.as_ref() == base && lhs.as_ref() == &Expr::Constant(-1))
    )
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
    fn combines_a_variable_with_its_predecessor_power_in_either_order() {
        let x = Expr::variable("x");
        let exponent = x.clone() + Expr::constant(-1);
        let expected = x.clone().pow_expr(x.clone());

        assert_eq!(x.clone() * x.clone().pow_expr(exponent.clone()), expected);
        assert_eq!(x.clone().pow_expr(exponent) * x.clone(), expected);
    }

    #[test]
    fn differentiates_x_to_the_x_with_base_derivative() {
        let x = Expr::variable("x");
        let expression = x.clone().pow_expr(x.clone());

        let derivative = differentiate(&expression, "x");
        // Expected: x^x + ln(x)*x^x
        assert_eq!(derivative.to_string(), "x^x + ln(x)*x^x");
    }

    #[test]
    fn evaluates_expression_with_scalar_variable_values() {
        let x = Expr::variable("x");
        let y = Expr::variable("y");
        let expression = Expr::constant(2) * x + y.pow(2);
        let values = HashMap::from([("x".to_string(), -2.0), ("y".to_string(), 3.0)]);

        assert_eq!(evaluate(&expression, &values), Ok(5.0));
    }

    #[test]
    fn substitutes_expressions_and_simplifies_trigonometric_identity() {
        let x = Expr::variable("x");
        let y = Expr::variable("y");
        let theta = Expr::variable("theta");
        let expression = x.pow(2) + y.pow(2);
        let replacements = HashMap::from([
            ("x".to_string(), theta.clone().cos()),
            ("y".to_string(), theta.clone().sin()),
        ]);
        let substituted = substitute(&expression, &replacements);

        assert_eq!(simplify(&substituted), Expr::constant(1));
    }

    #[test]
    fn does_not_simplify_trigonometric_squares_with_different_arguments() {
        let theta = Expr::variable("theta");
        let phi = Expr::variable("phi");
        let expression = theta.cos().pow(2) + phi.sin().pow(2);

        assert_eq!(simplify(&expression), expression);
    }

    #[test]
    fn evaluates_trigonometric_substitutions_to_one() {
        let x = Expr::variable("x");
        let y = Expr::variable("y");
        let expression = x.pow(2) + y.pow(2);
        let theta: f64 = 0.7;
        let values = HashMap::from([
            ("x".to_string(), theta.cos()),
            ("y".to_string(), theta.sin()),
        ]);

        assert!((evaluate(&expression, &values).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn evaluation_reports_missing_variables_and_division_by_zero() {
        let empty_values = HashMap::new();
        assert_eq!(
            evaluate(&Expr::variable("x"), &empty_values),
            Err(EvalError::VariableNotFound("x".to_string()))
        );
        assert_eq!(
            evaluate(&(Expr::constant(1) / Expr::constant(0)), &empty_values),
            Err(EvalError::DivisionByZero)
        );
    }
}
