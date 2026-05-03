use num_complex::Complex32;
use std::f32::consts::PI;
use std::f32::consts::SQRT_2;

fn main() {
    // Create array of complex numbers
    let complex_numbers = [
        Complex32::new(1.0, 0.0),
        Complex32::new(SQRT_2 / 2.0, SQRT_2 / 2.0),
        Complex32::new(0.0, 1.0),
        Complex32::new(-SQRT_2 / 2.0, SQRT_2 / 2.0),
        Complex32::new(-1.0, 0.0),
        Complex32::new(-SQRT_2 / 2.0, -SQRT_2 / 2.0),
        Complex32::new(0.0, -1.0),
        Complex32::new(SQRT_2 / 2.0, -SQRT_2 / 2.0),
    ];
    for (i, z) in complex_numbers.iter().enumerate() {
        println!("z[{}] = {}", i, z);
        println!("  abs = {}", z.norm());
        println!("  arg = {:.2}", z.arg() / PI * 180.0); // Convert to degrees
    }
}
