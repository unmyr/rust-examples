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
fn it_ndarray_2d_index_access() {
    // Example of index access to a two-dimensional array
    let mut w = ndarray::Array2::<u8>::zeros((3, 3));
    w[[1, 2]] = 42; // Set element at row 1, column 2
    assert_eq!(w[[1, 2]], 42);
}

// Gets a specific row view.
#[test]
fn it_ndarray_2d_get_a_specific_row_view() {
    use ndarray::arr2;

    let a = arr2(&[[1, 2], [3, 4], [5, 6]]);
    let row1 = a.row(1);
    println!("row(1)={:?}", row1);
    assert_eq!(&row1, &ndarray::array![3, 4]);
}

// Fill specific rows with zeros
#[test]
fn it_ndarray_2d_fill_specific_rows_with_zeros() {
    use ndarray::arr2;

    let a = arr2(&[[1, 2], [3, 4], [5, 6]]);
    let mut b = a.clone();
    let mut row_mut = b.row_mut(1);
    row_mut.fill(0);
    println!("b={:?}", b);
    assert_eq!(b, arr2(&[[1, 2], [0, 0], [5, 6]]));
}

#[test]
fn it_ndarray_array_elementwise_multiplication() {
    let v = ndarray::array![2, -4];
    println!("{:?}", &v * &v);
    assert_eq!(&v * &v, ndarray::array![4, 16]);
}

// Multiplies two matrices.
#[test]
fn it_ndarray_2d_matrix_multiplication_1() {
    use ndarray::arr2;

    let a = arr2(&[[1, 2, 1], [0, 1, 1]]);
    let b = arr2(&[[1, 0], [0, 1], [1, 1]]);
    let c = a.dot(&b);
    println!("{:?}", c);
    println!("{} {}\n{} {}", c[(0, 0)], c[(0, 1)], c[(1, 0)], c[(1, 1)]);
    assert_eq!(c[(0, 0)], 2);
    assert_eq!(c[(0, 1)], 3);
    assert_eq!(c[(1, 0)], 1);
    assert_eq!(c[(1, 1)], 2);
    assert_eq!(c, arr2(&[[2, 3], [1, 2]]));
}

#[test]
fn it_ndarray_2d_matrix_multiplication_2() {
    let w_1 = ndarray::array![[0.1, 0.2], [0.3, 0.4]];
    let x_1 = ndarray::array![1., 0.];
    let b_1 = ndarray::array![0.1, 0.1];
    assert_eq!(w_1.dot(&x_1) + b_1, ndarray::array![0.2, 0.4]);
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

// create an empty array and append rows
#[test]
fn it_ndarray_2d_add_rows() {
    let h_1 = ndarray::arr2(&[[1., 2.], [3., 4.]]);
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    let b_1 = ndarray::array![0.1, 0.1];

    let mut h1_out_results_nd = ndarray::Array2::zeros((0, 2));
    for in_view in test_inputs.rows() {
        let h1_out = (&h_1.dot(&in_view) + &b_1)
            .into_owned()
            .insert_axis(ndarray::Axis(0));
        let _ = h1_out_results_nd.push_row(h1_out.row(0));
        println!("h1_out={:?}", &h1_out);
    }
    println!("h1_out_results_nd={:?}", h1_out_results_nd);
    assert_eq!(h1_out_results_nd.row(0), ndarray::array![0.1, 0.1]);
    assert_eq!(h1_out_results_nd.row(1), ndarray::array![2.1, 4.1]);
    assert_eq!(h1_out_results_nd.row(2), ndarray::array![1.1, 3.1]);
    assert_eq!(h1_out_results_nd.row(3), ndarray::array![3.1, 7.1]);
    assert_eq!(
        h1_out_results_nd,
        ndarray::arr2(&[[0.1, 0.1], [2.1, 4.1], [1.1, 3.1], [3.1, 7.1]])
    );
}

// create an empty array and append columns
#[test]
fn it_ndarray_2d_add_columns() {
    let h_1 = ndarray::arr2(&[[1., 2.], [3., 4.]]);
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    let b_1 = ndarray::array![0.1, 0.1];

    let mut h1_out_results_nd = ndarray::Array2::zeros((2, 0));
    for in_view in test_inputs.rows() {
        let h1_out = (&h_1.dot(&in_view) + &b_1)
            .into_owned()
            .insert_axis(ndarray::Axis(0));
        let _ = h1_out_results_nd.push_column(h1_out.row(0));
        println!("h1_out={:?}", &h1_out);
    }
    println!("h1_out_results_nd={:?}", h1_out_results_nd);
    assert_eq!(h1_out_results_nd.column(0), ndarray::array![0.1, 0.1]);
    assert_eq!(h1_out_results_nd.column(1), ndarray::array![2.1, 4.1]);
    assert_eq!(h1_out_results_nd.column(2), ndarray::array![1.1, 3.1]);
    assert_eq!(h1_out_results_nd.column(3), ndarray::array![3.1, 7.1]);
    assert_eq!(
        h1_out_results_nd,
        ndarray::arr2(&[[0.1, 2.1, 1.1, 3.1], [0.1, 4.1, 3.1, 7.1]])
    );
}
