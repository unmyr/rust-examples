use ndarray::Array1;

#[test]
fn it_ndarray_mapv() {
    // Create a 1D array
    let arr = Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element using mapv
    let squared = arr.mapv(|x| x * x);
    assert_eq!(squared, Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}

#[test]
fn it_ndarray_mapv_inplace() {
    // Create a 1D array
    let mut arr = Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element in place using mapv_inplace
    arr.mapv_inplace(|x| x * x);
    assert_eq!(arr, Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}
