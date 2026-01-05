pub mod relu_functions;

/// Computes the L2 norm (Euclidean distance) between two points (x, y).
/// # Examples
/// ```rust
/// use generic_functions::l2_norm;
/// assert!((l2_norm(3.0_f32, 4.0_f32) - 5.0).abs() < 1e-6);
/// ```
pub fn l2_norm<T: num_traits::Float>(x: T, y: T) -> T {
    (x * x + y * y).sqrt()
}

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

/// Sum number in Vec
/// NOTE: Copy is used so we can pass values without moving them.
/// # Examples
/// ```rust
/// use generic_functions::sum_vec;
/// let v = vec![1, 2, 3];
/// assert_eq!(sum_vec(&v), 6);
/// ```
pub fn sum_vec<T>(v: &Vec<T>) -> T
where
    T: num_traits::identities::Zero + std::ops::Add + Copy,
{
    v.iter()
        .fold(T::zero(), |accumulator, &part| accumulator + part)
}
