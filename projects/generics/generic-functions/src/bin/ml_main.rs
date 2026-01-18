use generic_functions::ml_functions::sigmoid;

fn main() {
    // Test sigmoid function on f32 type
    println!("sigmoid(0.0) = {}", sigmoid(0.0_f32));

    // Alternatively, using turbofish syntax
    println!("sigmoid(0.0) = {}", sigmoid::<f32>(0.0));
}
