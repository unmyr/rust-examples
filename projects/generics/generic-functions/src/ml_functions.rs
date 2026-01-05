// Identity function (does nothing)
pub fn identity<T>(x: T) -> T {
    x
}

// The derivative of the identity function is always 1
pub fn identity_derivative<T: num_traits::identities::One>(_: T) -> T {
    T::one()
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
pub fn relu<T>(x: T) -> T
where
    T: core::cmp::PartialOrd + num_traits::identities::Zero,
{
    if x > T::zero() { x } else { T::zero() }
}

/// Computes the derivative of the ReLU function.
/// # Examples
/// ```
/// use generic_functions::ml_functions::relu_derivative;
/// assert_eq!(relu_derivative(-1), 0);
/// assert_eq!(relu_derivative(2), 1);
/// ```
pub fn relu_derivative<T>(x: T) -> T
where
    T: core::cmp::PartialOrd + num_traits::identities::Zero + num_traits::identities::One,
{
    if x > T::zero() { T::one() } else { T::zero() }
}

/// Computes the sigmoid function for a given input `x`.
/// # Examples
///
/// ```rust
/// use generic_functions::ml_functions::sigmoid;
/// assert!((sigmoid(0.0_f32) - 0.5).abs() < 1e-6);
/// ```
pub fn sigmoid<T: num_traits::Float>(x: T) -> T {
    T::one() / (T::one() + (-x).exp())
}

/// Computes the derivative of the sigmoid function for a given input `x`.
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::sigmoid_derivative;
/// assert!((sigmoid_derivative(0.0_f32) - 0.25).abs() < 1e-6);
/// ```
pub fn sigmoid_derivative<T: num_traits::Float>(x: T) -> T {
    let s = sigmoid(x);
    s * (T::one() - s)
}

/// Hyperbolic tangent (tanh) activation functions and their derivatives

/// Hyperbolic tangent (tanh) activation function
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::tanh;
/// assert!((tanh(0.0_f32) - 0.0).abs() < 1e-6);
/// ```
pub fn tanh<T: num_traits::Float>(x: T) -> T {
    x.tanh()
}

/// Derivative of the hyperbolic tangent (tanh) function
/// # Examples
/// ```rust
/// use generic_functions::ml_functions::tanh_derivative;
/// assert!((tanh_derivative(0.0_f32) - 1.0).abs() < 1e-6);
/// ```
pub fn tanh_derivative<T: num_traits::Float>(x: T) -> T {
    let t = x.tanh();
    T::one() - t * t
}
