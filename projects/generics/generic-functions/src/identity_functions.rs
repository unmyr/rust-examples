// Identity function (does nothing)
pub fn identity<T>(x: T) -> T {
    x
}

// The derivative of the identity function is always 1
pub fn identity_derivative<T: num_traits::identities::One>(_: T) -> T {
    T::one()
}
