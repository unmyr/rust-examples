
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
