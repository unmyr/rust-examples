// Identity function (does nothing)
pub fn identity<F>(x: F) -> F {
    x
}

// The derivative of the identity function is always 1
pub fn identity_derivative<F: num_traits::identities::One>(_: F) -> F {
    F::one()
}

// ReLU activation function and its derivative implemented as generic functions.

/// These functions can operate on any numeric type that supports comparison
/// and has defined zero and one values.
///
/// # Examples
/// ```
/// use generic_functions::ml_functions::relu;
/// assert_eq!(relu(-1), 0);
/// assert_eq!(relu(2), 2);
/// ```
pub fn relu<F>(x: F) -> F
where
    F: core::cmp::PartialOrd + num_traits::identities::Zero,
{
    if x > F::zero() {
        x
    } else {
        F::zero()
    }
}

/// Computes the derivative of the ReLU function.
/// # Examples
/// ```
/// use generic_functions::ml_functions::relu_derivative;
/// assert_eq!(relu_derivative(-1), 0);
/// assert_eq!(relu_derivative(2), 1);
/// ```
pub fn relu_derivative<F>(x: F) -> F
where
    F: core::cmp::PartialOrd + num_traits::identities::Zero + num_traits::identities::One,
{
    if x > F::zero() {
        F::one()
    } else {
        F::zero()
    }
}

/// Computes the sigmoid function for a given input `x`.
/// # Examples
///
/// ```rust
/// use generic_functions::ml_functions::sigmoid;
/// assert!((sigmoid(0.0_f32) - 0.5).abs() < 1e-6);
/// ```
pub fn sigmoid<F: num_traits::Float>(x: F) -> F {
    F::one() / (F::one() + (-x).exp())
}

/// Computes the derivative of the sigmoid function for a given input `x`.
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::sigmoid_derivative;
/// assert!((sigmoid_derivative(0.0_f32) - 0.25).abs() < 1e-6);
/// ```
pub fn sigmoid_derivative<F: num_traits::Float>(x: F) -> F {
    let s = sigmoid(x);
    s * (F::one() - s)
}

/// Hyperbolic tangent (tanh) activation functions and their derivatives

/// Hyperbolic tangent (tanh) activation function
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::tanh;
/// assert!((tanh(0.0_f32) - 0.0).abs() < 1e-6);
/// ```
pub fn tanh<F: num_traits::Float>(x: F) -> F {
    x.tanh()
}

/// Derivative of the hyperbolic tangent (tanh) function
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::tanh_derivative;
/// assert!((tanh_derivative(0.0_f32) - 1.0).abs() < 1e-6);
/// ```
pub fn tanh_derivative<F: num_traits::Float>(x: F) -> F {
    let t = x.tanh();
    F::one() - t * t
}
