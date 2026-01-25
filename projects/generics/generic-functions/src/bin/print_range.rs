use core::ops::RangeBounds;

/// Prints all numbers in the given range.
/// Works with `Range`, `RangeInclusive`, `RangeFrom`, `RangeTo`, etc.
fn print_range<T, R>(range: R)
where
    T: std::fmt::Display
        + Copy
        + PartialOrd
        + num_traits::One
        + num_traits::Zero
        + num_traits::cast::NumCast,
    R: RangeBounds<T> + IntoIterator,
{
    // Determine start and end from the range bounds
    let start = match range.start_bound() {
        core::ops::Bound::Included(&s) => s,
        core::ops::Bound::Excluded(&s) => s + T::one(),
        core::ops::Bound::Unbounded => T::zero(),
    };

    let end = match range.end_bound() {
        core::ops::Bound::Included(&e) => e + T::one(),
        core::ops::Bound::Excluded(&e) => e,
        core::ops::Bound::Unbounded => start + T::from(10).unwrap(), // arbitrary limit
    };

    // the trait `Step` is not implemented for `T` under rustc 1.93.0
    // for i in start..end {}

    // Emulates the `Step` trait
    let mut i = start;
    while (start..end).contains(&i) {
        print!("{} ", i);
        i = i + T::one();
    }
    println!();
}

fn main() {
    // Works with different range types
    print_range::<i32, _>(1..5); // Exclusive end
    print_range::<i32, _>(1..=5); // Inclusive end
    print_range::<i32, _>(3..); // From 3 to arbitrary limit

    // `RangeTo<i32>` is not an iterator
    // print_range::<i32, _>(..5); // Up to 5
}
