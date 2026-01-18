fn l2_norm<F: num_traits::Float>(x: F, y: F) -> F {
    (x * x + y * y).sqrt()
}

fn main() {
    println!("l2_norm= {}", l2_norm(3., 4.));
}
