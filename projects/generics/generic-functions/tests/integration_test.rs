#[test]
fn it_add() {
    use generic_functions::add;

    assert!(add(1, 2) == 3);
    assert!(add(1 as usize, 2 as usize) == 3 as usize);
    assert!(add(1., 2.) == 3.);
    assert!(add(1. as f32, 2. as f32) == 3.);
}

#[test]
fn it_mul() {
    use generic_functions::mul;

    assert!(mul(2, 3) == 6);
    assert!(mul(2 as usize, 3 as usize) == 6 as usize);
    assert!(mul(2., 3.) == 6.);
    assert!(mul(2. as f32, 3. as f32) == 6.);
}

#[test]
fn it_constant_one() {
    use generic_functions::constant_one;

    assert!(constant_one(0) == 1);
    assert!(constant_one(1 as usize) == 1 as usize);
    assert!(constant_one(1.) == 1.);
    assert!(constant_one(1. as f32) == 1.);
}

#[test]
fn it_constant_two_by_addition() {
    use generic_functions::constant_two_by_addition;

    assert!(constant_two_by_addition(0) == 2);
    assert!(constant_two_by_addition(1 as usize) == 2 as usize);
    assert!(constant_two_by_addition(1.) == 2.);
    assert!(constant_two_by_addition(1. as f32) == 2.);
}

#[test]
fn it_constant_neg_300_from_cast() {
    use generic_functions::constant_neg_300_from_cast;

    assert!(constant_neg_300_from_cast(0 as i8) == None);
    assert!(constant_neg_300_from_cast(0) == Some(-300));
    assert!(constant_neg_300_from_cast(1 as usize) == None);
    assert!(constant_neg_300_from_cast(1.) == Some(-300.));
    assert!(constant_neg_300_from_cast(1. as f32) == Some(-300.));
}

#[test]
fn it_step_function() {
    use generic_functions::step_function;

    // i32
    assert!(step_function(-1) == 0);
    assert!(step_function(0) == 1);
    assert!(step_function(1) == 1);

    // f64
    assert!(step_function(-1.) == 0.);
    assert!(step_function(0.) == 1.);
    assert!(step_function(1.) == 1.);
}

#[test]
fn it_l2_norm() {
    use generic_functions::l2_norm;

    let (x, y): (f32, f32) = (3., 4.);
    assert_eq!(l2_norm(x, y), 5.);
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
