// Test tanh function
#[test]
fn test_tanh() {
    use generic_functions::ml_functions::tanh;

    // Test tanh function on f32 and f64 types
    let x = 0.0f32;
    let y = tanh(x);
    assert!((y - 0.0).abs() < 1e-6);

    let x = 0.0f64;
    let y = tanh(x);
    assert!((y - 0.0).abs() < 1e-6);
}

// Test tanh_derivative function
#[test]
fn test_tanh_derivative() {
    use generic_functions::ml_functions::tanh_derivative;
    // Test tanh_derivative function on f32 and f64 types
    let x = 0.0f32;
    let y = tanh_derivative(x);
    assert!((y - 1.0).abs() < 1e-6);
    let x = 0.0f64;
    let y = tanh_derivative(x);
    assert!((y - 1.0).abs() < 1e-6);
}
