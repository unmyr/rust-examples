// Convert the result of the XOR function for a 2x4, 4-column vector into a 1x4, 4-scalar array.
#[test]
fn it_ndarray_2d_column_to_vector() {
    let xor_continuous = |x1, x2| x1 + x2 - 2. * x1 * x2;

    let train_inputs: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
    assert_eq!(train_inputs.dim(), (2, 4));
    assert_eq!(train_inputs.shape(), [2, 4]);

    let train_answers_1d_vec: ndarray::ArrayBase<
        ndarray::OwnedRepr<f32>,
        ndarray::Dim<[usize; 1]>,
        f32,
    > = train_inputs.map_axis(ndarray::Axis(0), |column| {
        xor_continuous(column[0], column[1])
    });
    assert_eq!(train_answers_1d_vec.dim(), (4));
    assert_eq!(train_answers_1d_vec.shape(), [4]);
    assert_eq!(train_answers_1d_vec, ndarray::arr1(&[0., 1., 1., 0.]));

    // reshape
    let train_answer_column_vec = train_answers_1d_vec.into_shape_with_order((1, 4)).unwrap();
    assert_eq!(train_answer_column_vec, ndarray::arr2(&[[0., 1., 1., 0.]]));
}

#[test]
fn it_ndarray_2d_accumulate_column_using_scale_add() {
    let mut accumulated_errors: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 0.], [0., 0.], [0., 0.]]).reversed_axes();
    let loss_terms: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 2.], [1., 1.]]).reversed_axes();

    assert_eq!(accumulated_errors.dim(), (2, 4));
    assert_eq!(accumulated_errors.shape(), [2, 4]);

    accumulated_errors
        .column_mut(2)
        .scaled_add(1., &loss_terms.column(2));
    assert_eq!(accumulated_errors.column(2), ndarray::arr1(&[1., 2.]));
    accumulated_errors
        .column_mut(2)
        .scaled_add(1., &loss_terms.column(2));
    assert_eq!(accumulated_errors.column(2), ndarray::arr1(&[2., 4.]));
}

#[test]
fn it_ndarray_2d_accumulate_column_using_dereference() {
    let mut accumulated_errors: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 0.], [0., 0.], [0., 0.]]).reversed_axes();
    let loss_terms: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 2.], [1., 1.]]).reversed_axes();

    assert_eq!(accumulated_errors.dim(), (2, 4));
    assert_eq!(accumulated_errors.shape(), [2, 4]);

    // Build fail: error[E0067]: invalid left-hand side of assignment
    // Temporary variables are invalid and cannot be build using `+=`.
    // accumulated_errors.column_mut(2) += &loss_terms.column(2);
    *&mut accumulated_errors.column_mut(2) += &loss_terms.column(2);
    assert_eq!(accumulated_errors.column(2), ndarray::arr1(&[1., 2.]));
    *&mut accumulated_errors.column_mut(2) += &loss_terms.column(2);
    assert_eq!(accumulated_errors.column(2), ndarray::arr1(&[2., 4.]));
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
    assert!(interaction == ndarray::Array1::from(vec![0, 0, 0, 1]));
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
