/// Zadeh's Min
pub fn and_zadeh<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    // the trait `Ord` is not implemented for `{float}`
    // x.min(y)
    if x < y { x } else { y }
}

/// Zadeh's Max
pub fn or_zadeh<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    // the trait `Ord` is not implemented for `{float}`
    // x.max(y)
    if x < y { y } else { x }
}

/// Product
pub fn and_product<T>(x: T, y: T) -> T
where
    T: std::ops::Mul<Output = T>,
{
    x * y
}

/// Probabilistic sum
pub fn or_prob_sum<T>(x: T, y: T) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
{
    x + y - x * y
}

/// Lukasiewicz AND
pub fn and_lukasiewicz<T>(x: T, y: T) -> T
where
    T: PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + num_traits::identities::Zero
        + num_traits::identities::One,
{
    // NOTE:: the trait `Ord` is not implemented for `{float}`
    // std::cmp::max(T::zero(), x + y - T::one())
    let c = x + y - T::one();
    if c < T::zero() { T::zero() } else { c }
}

/// Lukasiewicz OR
pub fn or_lukasiewicz<T>(x: T, y: T) -> T
where
    T: PartialOrd + std::ops::Add<Output = T> + num_traits::identities::One,
{
    // the trait `Ord` is not implemented for `{float}`
    // T::one().min(x + y)
    let x_plus_y = x + y;
    if x_plus_y < T::one() {
        x_plus_y
    } else {
        T::one()
    }
}

// Drastic
pub fn or_drastic<T>(x: T, y: T) -> T
where
    T: PartialEq + Ord + num_traits::identities::Zero + num_traits::identities::One,
{
    if x == T::zero() {
        y
    } else if x == T::one() {
        T::one()
    } else {
        x.max(y)
    }
}

pub fn xor_continuous<T>(x: T, y: T) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul
        + num_traits::identities::One,
{
    x + y - (T::one() + T::one()) * x * y
}
