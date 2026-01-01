use ndarray::arr2;

fn main() {
    // Multiplying matrices
    let a = arr2(&[[1, 2, 1], [0, 1, 1]]);
    let b = arr2(&[[1, 0], [0, 1], [1, 1]]);
    let c = a.dot(&b);
    println!("{}", c);
    println!("{} {}\n{} {}", c[(0, 0)], c[(0, 1)], c[(1, 0)], c[(1, 1)]);
    assert_eq!(c, arr2(&[[2, 3], [1, 2]]));
}
