// Element-wise squaring with mapv
#[test]
fn it_ndarray_1d_mapv() {
    // Create a 1D array
    let arr = ndarray::Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element using mapv
    let squared = arr.mapv(|x| x * x);
    assert_eq!(squared, ndarray::Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}

// Element-wise squaring in place using `mapv`
#[test]
fn it_ndarray_1d_mapv_inplace() {
    // Create a 1D array
    let mut arr = ndarray::Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element in place using mapv_inplace
    arr.mapv_inplace(|x| x * x);
    assert_eq!(arr, ndarray::Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}

// Element-wise addition using `broadcast`
// See: https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#broadcasting
#[test]
fn it_ndarray_1d_broadcast_add() {
    let a1 = ndarray::arr1(&[1, 2, 3, 4, 5]);
    let b1 = ndarray::arr1(&[1]);
    let c1 = ndarray::arr1(&[2, 3, 4, 5, 6]);
    assert!(c1 == &a1 + &b1);
}

// Element-wise multiplication using `broadcast`
// See: https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#broadcasting
#[test]
fn it_ndarray_1d_broadcast_mul() {
    let a1 = ndarray::arr1(&[1, 2, 3, 4, 5]);
    let b1 = ndarray::arr1(&[2]);
    let c1 = ndarray::arr1(&[2, 4, 6, 8, 10]);
    assert!(c1 == &a1 * &b1);
}

// Element-wise multiplication
#[test]
fn it_ndarray_1d_elementwise_multiplication() {
    let v = ndarray::array![2, -4];
    println!("{:?}", &v * &v);
    assert_eq!(&v * &v, ndarray::array![4, 16]);
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

#[test]
fn it_ndarray_1d_product() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::Array1::<i32>::zeros(0);
    assert!(arr1_empty_i32.shape() == &[0]);
    assert!(arr1_empty_i32.product() == 1);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 4, 5, 6]);
    let arr1_f32 = arr1_i32.mapv(|v| v as f32);
    assert_eq!(arr1_i32.product(), 720);
    assert_eq!(arr1_f32.product(), 720.);
}
