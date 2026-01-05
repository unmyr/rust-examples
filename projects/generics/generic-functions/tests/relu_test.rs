#[test]
fn it_relu() {
    use generic_functions::relu_functions::relu;

    // Test various cases for ReLU function on integers
    assert!(relu(-1) == 0);
    assert!(relu(0) == 0);
    assert!(relu(1) == 1);
    assert!(relu(2) == 2);

    // Test various cases for ReLU function on floating-point numbers
    assert!((relu(-1.5_f32) - 0.0).abs() < 1e-6);
    assert!((relu(0.0_f32) - 0.0).abs() < 1e-6);
    assert!((relu(2.5_f32) - 2.5).abs() < 1e-6);
}

#[test]
fn it_relu_derivative() {
    use generic_functions::relu_functions::relu_derivative;

    // Test various cases for ReLU derivative on integers
    assert!(relu_derivative(-1) == 0);
    assert!(relu_derivative(0) == 0);
    assert!(relu_derivative(1) == 1);
    assert!(relu_derivative(2) == 1);

    // Test various cases for ReLU derivative on floating-point numbers
    assert!((relu_derivative(-1.5_f32) - 0.0).abs() < 1e-6);
    assert!((relu_derivative(0.0_f32) - 0.0).abs() < 1e-6);
    assert!((relu_derivative(2.5_f32) - 1.0).abs() < 1e-6);
}
