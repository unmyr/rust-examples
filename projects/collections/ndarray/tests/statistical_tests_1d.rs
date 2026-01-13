// Sum the elements of a 1-dimensional vector
#[test]
fn it_ndarray_1d_sum() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::Array1::<i32>::zeros(0);
    assert!(arr1_empty_i32.sum() == 0);
    assert!(arr1_empty_i32.shape() == &[0]);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 4, 5, 6]);
    let arr1_f32 = arr1_i32.mapv(|v| v as f32);
    assert_eq!(arr1_i32.sum(), 21);
    assert_eq!(arr1_f32.sum(), 21.);
}

// Averages the elements of a 1-dimensional vector
#[test]
fn it_ndarray_1d_mean() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::arr1::<i32>(&[]);
    assert_eq!(arr1_empty_i32.shape(), &[0]);
    assert_eq!(arr1_empty_i32.mean(), None);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 1, 3, 5]);
    let arr1_f32 = ndarray::arr1::<f32>(&[1., 2., 3., 1., 3., 5.]);
    assert_eq!(arr1_i32.shape(), &[6]);
    assert_eq!(arr1_i32.mean(), Some(2));
    assert_eq!(arr1_f32.mean(), Some(2.5));
}

// Population variance
#[test]
fn it_ndarray_1d_var() {
    let arr = ndarray::arr1::<f32>(&[2., 3., 3., 3., 9.]);
    assert_eq!(arr.shape(), &[arr.len()]);
    assert_eq!(arr.dim(), arr.len());

     let arr_mean: f32 = &arr.sum() / (arr.len() as f32);
    assert_eq!(arr.mean(), Some(arr_mean));

    let arr_var = arr.mapv(|v| v - arr_mean).mapv(|v| v * v).sum() / (arr.len() as f32);
    assert_eq!(arr.var(0.), arr_var);

    let arr_var = arr.mapv(|v| v * v).sum() / (arr.len() as f32) - arr.mean().unwrap().powf(2.);
    assert!((arr.var(0.) - arr_var).abs() < 0.01);
}
