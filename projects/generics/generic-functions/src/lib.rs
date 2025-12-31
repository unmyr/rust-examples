
/// Computes the sigmoid function for a given input `x`.
/// # Examples
///
/// ```rust
/// use generic_functions::sigmoid;
/// assert!((sigmoid(0.0_f32) - 0.5).abs() < 1e-6);
/// ```
pub fn sigmoid<T: num_traits::Float>(x: T) -> T {
    T::one() / (T::one() + (-x).exp())
}

#[test]
fn it_sigmoid() {
    let x = 0.0f32;
    let y = sigmoid(x);
    assert!((y - 0.5).abs() < 1e-6);

    let x = 0.0f64;
    let y = sigmoid(x);
    assert!((y - 0.5).abs() < 1e-6);
}
