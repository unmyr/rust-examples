extern crate nalgebra as na;

fn main() {
    let a = na::Matrix2x3::new(
        1, 2, 1,
        0, 1, 1
    );
    let b = na::Matrix3x2::new(
        1, 0,
        0, 1,
        1, 1
    );
    let c = a * b;
    println!("{:?}", c);
    println!(
        "{} {}\n{} {}",
        c[(0, 0)], c[(0, 1)], c[(1, 0)], c[(1, 1)]
    );
    assert_eq!(c, na::Matrix2::new(2, 3, 1, 2));
}
