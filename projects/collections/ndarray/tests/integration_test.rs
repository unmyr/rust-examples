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

#[test]
fn it_ndarray_2d_extend_column_1() {
    use ndarray::Array2;

    // Create a 2D array
    let test_inputs = ndarray::array![[0, 0], [0, 1], [1, 0], [1, 1]];
    // Extend the array by adding a new column which is the product of the first two columns
    let mut out_arr3 = Array2::<i32>::zeros((test_inputs.nrows(), 3));
    for (i, row) in test_inputs.rows().into_iter().enumerate() {
        let mut v = row.to_vec();
        v.push(v[0] * v[1]); // interaction term
        out_arr3.row_mut(i).assign(&ndarray::Array1::from(v));
    }
    assert_eq!(
        out_arr3,
        Array2::from_shape_vec((4, 3), vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1]).unwrap()
    );
}

#[test]
fn it_ndarray_2d_extend_column_2() {
    use ndarray::Array2;

    // Create a 2D array
    let test_inputs = ndarray::array![[0, 0], [0, 1], [1, 0], [1, 1]];
    assert!(test_inputs.shape() == &[4, 2]);

    // Calculate the interaction term, and then concatenate it as a new column
    let interaction = &test_inputs.column(0) * &test_inputs.column(1);
    assert!(interaction == Array1::from(vec![0, 0, 0, 1]));
    assert!(interaction.shape() == &[4]);

    // Convert interaction to a 2D column array
    let intersection_col = interaction.insert_axis(ndarray::Axis(1));
    assert!(intersection_col.shape() == &[4, 1]);

    // Concatenate the new column to the original array
    let out_arr3 = ndarray::concatenate![ndarray::Axis(1), test_inputs, intersection_col];

    assert_eq!(
        out_arr3,
        Array2::from_shape_vec((4, 3), vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1]).unwrap()
    );
    assert_eq!(out_arr3.shape(), &[4, 3]);
}
