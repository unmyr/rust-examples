pub mod logical_functions;
pub mod ml_functions;

/// Add two numbers.
pub fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

/// Multiply two numbers.
pub fn mul<T: std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

/// A constant function of 1.
pub fn constant_one<T: num_traits::identities::One>(_: T) -> T {
    T::one()
}

/// Two constant functions implemented using `T::one()`.
pub fn constant_two_by_addition<T>(_: T) -> T
where
    T: num_traits::identities::One + std::ops::Add<Output = T>,
{
    T::one() + T::one()
}

/// Two constant functions implemented using `T::from()`.
pub fn constant_neg_300_from_cast<T: num_traits::cast::NumCast>(_: T) -> Option<T> {
    T::from(-300)
}

/// A generic step function: returns T::zero() if input < T::zero(), else T::one().
pub fn step_function<T>(x: T) -> T
where
    T: PartialOrd + num_traits::identities::Zero + num_traits::identities::One,
{
    if x < T::zero() { T::zero() } else { T::one() }
}

/// Computes the L2 norm (Euclidean distance) between two points (x, y).
/// # Examples
/// ```rust
/// use generic_functions::l2_norm;
/// assert!((l2_norm(3.0_f32, 4.0_f32) - 5.0).abs() < 1e-6);
/// ```
pub fn l2_norm<T: num_traits::Float>(x: T, y: T) -> T {
    (x * x + y * y).sqrt()
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
