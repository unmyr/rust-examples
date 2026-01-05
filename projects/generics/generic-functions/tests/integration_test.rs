#[test]
fn it_l2_norm() {
    use generic_functions::l2_norm;

    let (x, y): (f32, f32) = (3., 4.);
    assert_eq!(l2_norm(x, y), 5.);
}

#[test]
fn it_sigmoid() {
    use generic_functions::sigmoid;

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

#[test]
fn it_vec_sum() {
    use generic_functions::sum_vec;

    let v_i32 = vec![-1, 0, 1, 2, 3];
    assert_eq!(sum_vec(&v_i32), 5);
    assert_eq!(sum_vec(&v_i32), 5);

    let v_usize: Vec<usize> = vec![0, 1, 2, 3];
    assert_eq!(sum_vec(&v_usize), 6);

    let v_f32: Vec<f32> = vec![1., 2., 3.];
    assert_eq!(sum_vec(&v_f32), 6_f32);

    let v_f64 = vec![1., 2., 3.];
    assert_eq!(sum_vec(&v_f64), 6.);
}
