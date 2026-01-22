#[test]
fn it_ndarray_2d_eyes() {
    let eye3 = ndarray::Array2::<f32>::eye(3);
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 = ndarray::Array2::from_diag_elem(3, 1.);
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 = ndarray::Array2::from_diag(&ndarray::arr1(&[1., 1., 1.]));
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let mut eye3 = ndarray::Array2::<f32>::zeros((3, 3));
    (0_usize..eye3.dim().0).for_each(|i| {
        eye3[[i, i]] = 1.;
    });
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 = ndarray::Array2::<f32>::from_elem((3, 3), 1_f32);
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 = ndarray::Array::from_shape_fn((3, 3), |(i, j)| if i == j { 1_f32 } else { 0_f32 });
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 = ndarray::Array::from_shape_fn((3, 3), |(i, j)| if i == j { 1_f32 } else { 0_f32 });
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );

    let eye3 =
        ndarray::Array::from_shape_vec((3, 3), vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]).unwrap();
    println!("{:?}", eye3);
    assert_eq!(
        eye3,
        ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    );
}
