/// Hyperbolic tangent (tanh) activation functions and their derivatives

/// Hyperbolic tangent (tanh) activation function
/// # Examples
/// ```rust
/// use generic_functions::tanh_functions::tanh;
/// assert!((tanh(0.0_f32) - 0.0).abs() < 1e-6);
/// ```
pub fn tanh<T: num_traits::Float>(x: T) -> T {
    x.tanh()
}

/// Derivative of the hyperbolic tangent (tanh) function
/// # Examples
/// ```rust
/// use generic_functions::tanh_functions::tanh_derivative;
/// assert!((tanh_derivative(0.0_f32) - 1.0).abs() < 1e-6);
/// ```
pub fn tanh_derivative<T: num_traits::Float>(x: T) -> T {
    let t = x.tanh();
    T::one() - t * t
}
