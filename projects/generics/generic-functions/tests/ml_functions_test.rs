// Test ML functions

// Test sigmoid function
#[test]
fn it_sigmoid() {
    use generic_functions::ml_functions::sigmoid;

    // Test sigmoid function on f32 and f64 types
    let x = 0.0f32;
    let y = sigmoid(x);
    assert!((y - 0.5).abs() < 1e-6);

    let x = 0.0f64;
    let y = sigmoid(x);
    assert!((y - 0.5).abs() < 1e-6);

    // Alternatively, using turbofish syntax
    let y = sigmoid::<f32>(0.0);
    assert!((y - 0.5).abs() < 1e-6);
}

// Test sigmoid_derivative function
#[test]
fn it_sigmoid_derivative() {
    use generic_functions::ml_functions::sigmoid_derivative;
    // Test sigmoid_derivative function on f32 and f64 types
    let x = 0.0f32;
    let y = sigmoid_derivative(x);
    assert!((y - 0.25).abs() < 1e-6);
    let x = 0.0f64;
    let y = sigmoid_derivative(x);
    assert!((y - 0.25).abs() < 1e-6);
}
