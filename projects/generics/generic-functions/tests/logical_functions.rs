#[test]
fn it_and_zadeh() {
    use generic_functions::logical_functions::and_zadeh;

    // i32
    assert_eq!(and_zadeh(0, 0), 0);
    assert_eq!(and_zadeh(0, 1), 0);
    assert_eq!(and_zadeh(1, 0), 0);
    assert_eq!(and_zadeh(1, 1), 1);

    // f64
    assert_eq!(and_zadeh(0., 0.), 0.);
    assert_eq!(and_zadeh(0., 1.), 0.);
    assert_eq!(and_zadeh(1., 0.), 0.);
    assert_eq!(and_zadeh(1., 1.), 1.);
}

#[test]
fn it_or_zadeh() {
    use generic_functions::logical_functions::or_zadeh;

    // i32
    assert_eq!(or_zadeh(0, 0), 0);
    assert_eq!(or_zadeh(0, 1), 1);
    assert_eq!(or_zadeh(1, 0), 1);
    assert_eq!(or_zadeh(1, 1), 1);

    // f64
    assert_eq!(or_zadeh(0., 0.), 0.);
    assert_eq!(or_zadeh(0., 1.), 1.);
    assert_eq!(or_zadeh(1., 0.), 1.);
    assert_eq!(or_zadeh(1., 1.), 1.);
}

#[test]
fn it_and_product() {
    use generic_functions::logical_functions::and_product;

    // i32
    assert_eq!(and_product(0, 0), 0);
    assert_eq!(and_product(0, 1), 0);
    assert_eq!(and_product(1, 0), 0);
    assert_eq!(and_product(1, 1), 1);

    // f64
    assert_eq!(and_product(0., 0.), 0.);
    assert_eq!(and_product(0., 1.), 0.);
    assert_eq!(and_product(1., 0.), 0.);
    assert_eq!(and_product(1., 1.), 1.);
}

#[test]
fn it_and_lukasiewicz() {
    use generic_functions::logical_functions::and_lukasiewicz;

    // i32
    assert_eq!(and_lukasiewicz(0, 0), 0);
    assert_eq!(and_lukasiewicz(0, 1), 0);
    assert_eq!(and_lukasiewicz(1, 0), 0);
    assert_eq!(and_lukasiewicz(1, 1), 1);

    // f64
    assert_eq!(and_lukasiewicz(0., 0.), 0.);
    assert_eq!(and_lukasiewicz(0., 1.), 0.);
    assert_eq!(and_lukasiewicz(1., 0.), 0.);
    assert_eq!(and_lukasiewicz(1., 1.), 1.);
}

#[test]
fn it_or_lukasiewicz() {
    use generic_functions::logical_functions::or_lukasiewicz;

    // i32
    assert_eq!(or_lukasiewicz(0, 0), 0);
    assert_eq!(or_lukasiewicz(0, 1), 1);
    assert_eq!(or_lukasiewicz(1, 0), 1);
    assert_eq!(or_lukasiewicz(1, 1), 1);

    // f64
    assert_eq!(or_lukasiewicz(0., 0.), 0.);
    assert_eq!(or_lukasiewicz(0., 1.), 1.);
    assert_eq!(or_lukasiewicz(1., 0.), 1.);
    assert_eq!(or_lukasiewicz(1., 1.), 1.);
}
