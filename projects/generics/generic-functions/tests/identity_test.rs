// Test identity function
#[test]
fn it_identity() {
    use generic_functions::identity_functions::identity;
    assert_eq!(identity(5), 5);
    assert_eq!(identity("hello"), "hello");
    assert_eq!(identity(3.14_f32), 3.14_f32);
}

// Test identity function with references
#[test]
fn it_identity_ref() {
    use generic_functions::identity_functions::identity;
    let x = 10;
    assert_eq!(identity(&x), &10);
    let s = "world";
    assert_eq!(identity(&s), &"world");
}
// Test identity function with custom struct
#[test]
fn it_identity_custom_struct() {
    use generic_functions::identity_functions::identity;
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    assert_eq!(identity(p), Point { x: 1, y: 2 });
}

// Test identity function with option type
#[test]
fn it_identity_option() {
    use generic_functions::identity_functions::identity;
    let some_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;
    assert_eq!(identity(some_value), Some(100));
    assert_eq!(identity(none_value), None);
}
// Test identity function with vectors
#[test]
fn it_identity_vector() {
    use generic_functions::identity_functions::identity;
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(identity(vec.clone()), vec);
}
// Test identity function with tuples
#[test]
fn it_identity_tuple() {
    use generic_functions::identity_functions::identity;
    let tuple = (1, "tuple", 3.14_f32);
    assert_eq!(identity(tuple), (1, "tuple", 3.14_f32));
}
// Test identity function with arrays
#[test]
fn it_identity_array() {
    use generic_functions::identity_functions::identity;
    let array = [10, 20, 30, 40];
    assert_eq!(identity(array), [10, 20, 30, 40]);
}
// Test identity_derivative function
#[test]
fn it_identity_derivative() {
    use generic_functions::identity_functions::identity_derivative;
    assert_eq!(identity_derivative(5), 1);
    assert_eq!(identity_derivative(3.14_f32), 1.0_f32);
}
