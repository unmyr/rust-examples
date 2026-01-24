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

    let a = arr2(&[[1, 2], [3, 4], [5, 6], [7, 8]]);
    let mut b = a.clone();

    let row = b.row_mut(0);
    row.mapv_into(|v| 2 * v);

    let mut row_mut = b.row_mut(1);
    row_mut.fill(0);

    let mut row_mut = b.row_mut(2);
    row_mut.assign(&ndarray::array![-5, -6].view());

    let mut row_mut = b.row_mut(3);
    row_mut.assign(&ndarray::arr1(&[-7, -8]).view());

    println!("b={:?}", b);
    assert_eq!(b, arr2(&[[2, 4], [0, 0], [-5, -6], [-7, -8]]));
}

// Convert the result of the XOR function for four row vectors (4x2) into four scalar arrays (4x1).
#[test]
fn it_ndarray_2d_row_to_vector() {
    let xor_continuous = |x1, x2| x1 + x2 - 2. * x1 * x2;

    let train_inputs: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    assert_eq!(train_inputs.dim(), (4, 2));
    assert_eq!(train_inputs.shape(), [4, 2]);

    let train_answers_1d_vec =
        train_inputs.map_axis(ndarray::Axis(1), |row| xor_continuous(row[0], row[1]));
    assert_eq!(train_answers_1d_vec.dim(), (4));
    assert_eq!(train_answers_1d_vec.shape(), [4]);
    assert_eq!(train_answers_1d_vec, ndarray::arr1(&[0., 1., 1., 0.]));

    // reshape
    let train_answer_row_vec = train_answers_1d_vec.into_shape_with_order((4, 1)).unwrap();
    assert_eq!(
        train_answer_row_vec,
        ndarray::arr2(&[[0.], [1.], [1.], [0.]])
    );
}

// Multiply each row of the matrix by [1, 2]
#[test]
fn it_ndarray_2d_broadcast_multiply_each_row() {
    use ndarray::arr2;

    let a = arr2(&[[1., 1.], [1., 2.], [3., 3.], [4., 4.]]);
    let b = arr2(&[[1., 2.]]);
    let c = arr2(&[[1., 2.], [1., 4.], [3., 6.], [4., 8.]]);
    println!("{:?}", &a * &b);
    assert!(c == &a * &b);
}

#[test]
fn it_ndarray_2d_extend_column_using_iter() {
    use ndarray::Array2;

    // Create a 2D array
    let train_inputs_nxk = ndarray::array![[0, 0], [0, 1], [1, 0], [1, 1]];
    // Extend the array by adding a new column which is the product of the first two columns
    let mut train_inputs_with_interaction_nxk = Array2::<i32>::zeros((train_inputs_nxk.nrows(), 3));
    for (i, row) in train_inputs_nxk.rows().into_iter().enumerate() {
        let mut v = row.to_vec();
        v.push(v[0] * v[1]); // interaction term
        train_inputs_with_interaction_nxk
            .row_mut(i)
            .assign(&ndarray::Array1::from(v));
    }
    assert_eq!(
        train_inputs_with_interaction_nxk,
        ndarray::arr2(&[[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 1]])
    );
}

#[test]
fn it_ndarray_2d_extend_column_using_concatenate() {
    use ndarray::Array2;

    // Create a 2D array
    let train_inputs_nxk = ndarray::array![[0, 0], [0, 1], [1, 0], [1, 1]];
    assert!(train_inputs_nxk.shape() == &[4, 2]);

    // Calculate the interaction term, and then concatenate it as a new column
    let interaction_col_n = &train_inputs_nxk.column(0) * &train_inputs_nxk.column(1);
    assert!(interaction_col_n == ndarray::Array1::from(vec![0, 0, 0, 1]));
    assert!(interaction_col_n.shape() == &[4]);

    // Convert interaction to a 2D column array
    let intersection_col_nxk = interaction_col_n.insert_axis(ndarray::Axis(1));
    assert!(intersection_col_nxk.shape() == &[4, 1]);

    // Concatenate the new column to the original array
    let train_inputs_with_interaction_nxk =
        ndarray::concatenate![ndarray::Axis(1), train_inputs_nxk, intersection_col_nxk];

    assert_eq!(
        train_inputs_with_interaction_nxk,
        Array2::from_shape_vec((4, 3), vec![0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1]).unwrap()
    );
    assert_eq!(train_inputs_with_interaction_nxk.shape(), &[4, 3]);
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
