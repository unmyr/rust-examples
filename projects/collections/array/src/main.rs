fn main() {
    let a: [f32; 3] = [-0.5, -0.5, 0.0];
    println!("a = {:?}", a);
    let (min, max): ([f32; 3], [f32; 3]) = ([-0.5, -0.5, 0.0], [0.5, 0.5, 0.0]);
    println!("min = {:?}, max = {:?}", min, max);
}
