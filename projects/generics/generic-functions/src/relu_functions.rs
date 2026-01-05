// ReLU activation function and its derivative implemented as generic functions.

/// These functions can operate on any numeric type that supports comparison
/// and has defined zero and one values.
/// 
/// # Examples
/// ```
/// use generic_functions::relu_functions::relu;
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
/// use generic_functions::relu_functions::relu_derivative;
/// assert_eq!(relu_derivative(-1), 0);
/// assert_eq!(relu_derivative(2), 1);
/// ```
pub fn relu_derivative<T>(x: T) -> T
where
    T: core::cmp::PartialOrd + num_traits::identities::Zero + num_traits::identities::One,
{
    if x > T::zero() { T::one() } else { T::zero() }
}
