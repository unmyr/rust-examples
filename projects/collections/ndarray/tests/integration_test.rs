use ndarray::Array1;

#[test]
fn it_ndarray_1d_mapv() {
    // Create a 1D array
    let arr = Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element using mapv
    let squared = arr.mapv(|x| x * x);
    assert_eq!(squared, Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}

#[test]
fn it_ndarray_1d_mapv_inplace() {
    // Create a 1D array
    let mut arr = Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element in place using mapv_inplace
    arr.mapv_inplace(|x| x * x);
    assert_eq!(arr, Array1::from(vec![1, 4, 9, 16, 25]));
    assert_eq!(arr.len(), 5);
}

#[test]
fn it_ndarray_1d_2d_mean() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::arr1::<i32>(&[]);
    assert_eq!(arr1_empty_i32.shape(), &[0]);
    assert_eq!(arr1_empty_i32.mean(), None);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 1, 3, 5]);
    let arr1_f32 = ndarray::arr1::<f32>(&[1., 2., 3., 1., 3., 5.]);
    assert_eq!(arr1_i32.shape(), &[6]);
    assert_eq!(arr1_i32.mean(), Some(2));
    assert_eq!(arr1_f32.mean(), Some(2.5));

    // Create a 2D array
    let arr2_empty_i32 = ndarray::arr2::<i32, 2>(&[]);
    assert_eq!(arr2_empty_i32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_i32.mean(), None);

    let arr2_i32 = ndarray::arr2(&[[1, 2], [3, 1], [3, 5]]);
    let arr2_f32 = ndarray::arr2::<f32, 2>(&[[1., 2.], [3., 1.], [3., 5.]]);
    assert_eq!(arr2_i32.shape(), &[3, 2]);
    assert_eq!(arr2_i32.mean(), Some(2));
    assert_eq!(arr2_f32.shape(), &[3, 2]);
    assert_eq!(arr2_f32.mean(), Some(2.5));
}

#[test]
fn it_ndarray_1d_2d_sum() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::Array1::<i32>::zeros(0);
    assert!(arr1_empty_i32.sum() == 0);
    assert!(arr1_empty_i32.shape() == &[0]);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 4, 5, 6]);
    let arr1_f32 = arr1_i32.mapv(|v| v as f32);
    assert_eq!(arr1_i32.sum(), 21);
    assert_eq!(arr1_f32.sum(), 21.);

    // Create a 2D array
    let arr2_empty_i32 = ndarray::Array2::<i32>::zeros((0, 2));
    assert_eq!(arr2_empty_i32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_i32.sum(), 0);
    let arr2_empty_f32 = ndarray::Array2::<f32>::zeros((0, 2));
    assert_eq!(arr2_empty_f32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_f32.sum(), 0.);

    let arr2_i32 = ndarray::arr2(&[[1, 2], [3, 4], [5, 6]]);
    let arr2_f32 = arr2_i32.mapv(|v| v as f32);
    assert_eq!(arr2_i32.shape(), &[3, 2]);
    assert_eq!(arr2_i32.sum(), 21);
    assert_eq!(arr2_f32.shape(), &[3, 2]);
    assert_eq!(arr2_f32.sum(), 21.);
}

#[test]
fn it_ndarray_1d_2d_product() {
    // Create a 1D array
    let arr1_empty_i32 = ndarray::Array1::<i32>::zeros(0);
    assert!(arr1_empty_i32.shape() == &[0]);
    assert!(arr1_empty_i32.product() == 1);

    let arr1_i32 = ndarray::arr1(&[1, 2, 3, 4, 5, 6]);
    let arr1_f32 = arr1_i32.mapv(|v| v as f32);
    assert_eq!(arr1_i32.product(), 720);
    assert_eq!(arr1_f32.product(), 720.);

    // Create a 2D array
    let arr2_empty_i32 = ndarray::Array2::<i32>::zeros((0, 2));
    assert_eq!(arr2_empty_i32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_i32.product(), 1);
    let arr2_empty_f32 = ndarray::Array2::<f32>::zeros((0, 2));
    assert_eq!(arr2_empty_f32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_f32.product(), 1.);

    let arr2_i32 = ndarray::arr2(&[[1, 2], [3, 4], [5, 6]]);
    let arr2_f32 = arr2_i32.mapv(|v| v as f32);
    assert_eq!(arr2_i32.shape(), &[3, 2]);
    assert_eq!(arr2_i32.product(), 720);
    assert_eq!(arr2_f32.shape(), &[3, 2]);
    assert_eq!(arr2_f32.product(), 720.);
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

// Replace all elements in a 3×2 matrix
#[test]
fn it_ndarray_2d_replace_all_elements() {
    // Create a 3x2 matrix
    let mut m: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>, i32> =
        ndarray::arr2(&[[1, 2], [3, 4], [5, 6]]);
    assert_eq!(&m.shape(), &[3, 2]);
    assert_eq!(&m.dim(), &(3, 2));

    // Replace with new values from a ndarray
    let new_m = ndarray::arr2(&[[10, 20], [30, 40], [50, 60]]);
    m.assign(&new_m);
    println!("\nAfter assigning new values:\n{m}");
    assert_eq!(&m, ndarray::arr2(&[[10, 20], [30, 40], [50, 60]]));
    assert_eq!(&m.row(0).view(), &ndarray::arr1(&[10, 20]).view());
    assert_eq!(&m.row(1).view(), &ndarray::arr1(&[30, 40]).view());
    assert_eq!(&m.row(2).view(), &ndarray::arr1(&[50, 60]).view());

    // Create a 3x2 matrix
    let mut m: ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>, i32> =
        ndarray::arr2(&[[1, 2], [3, 4], [5, 6]]);
    assert_eq!(&m.shape(), &[3, 2]);
    assert_eq!(&m.dim(), &(3, 2));

    // Replace with new values from a vector
    let new_values = vec![10, 20, 30, 40, 50, 60];
    let new_m = &ndarray::Array2::from_shape_vec(m.dim(), new_values).unwrap();
    m.assign(&new_m);
    println!("\nAfter assigning new values:\n{m}");
    assert_eq!(&m, ndarray::arr2(&[[10, 20], [30, 40], [50, 60]]));
    assert_eq!(&m.row(0).view(), &ndarray::arr1(&[10, 20]).view());
    assert_eq!(&m.row(1).view(), &ndarray::arr1(&[30, 40]).view());
    assert_eq!(&m.row(2).view(), &ndarray::arr1(&[50, 60]).view());
}

#[test]
fn it_ndarray_array_elementwise_multiplication() {
    let v = ndarray::array![2, -4];
    println!("{:?}", &v * &v);
    assert_eq!(&v * &v, ndarray::array![4, 16]);
}

// broadcasting
// See: https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#broadcasting
#[test]
fn it_ndarray_2d_broadcast_add() {
    use ndarray::arr2;

    // We can add because the shapes are compatible even if not equal.
    // The `b` array is shape 1 × 2 but acts like a 4 × 2 array.
    let a = arr2(&[[1., 1.], [1., 2.], [3., 3.], [4., 4.]]);
    let b = arr2(&[[0., 1.]]);
    let c = arr2(&[[1., 2.], [1., 3.], [3., 4.], [4., 5.]]);
    assert!(c == &a + &b);
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

// Multiplies two matrices.
#[test]
fn it_ndarray_2d_matrix_multiplication_m22_m21() {
    // use ndarray::{arr1,arr2};

    let a = ndarray::arr2(&[[1, 2], [3, 4]]);
    // column vector
    let b_col_v = ndarray::arr2(&[[1], [2]]);
    let c = &a.dot(&b_col_v);
    assert_eq!(&a.shape(), &[2, 2]);
    assert_eq!(&b_col_v.shape(), &[2, 1]);
    assert_eq!(&c.shape(), &[2, 1]);
    assert_eq!(
        c,
        ndarray::arr2(&[
            [a[(0, 0)] * b_col_v[(0, 0)] + a[(0, 1)] * b_col_v[(1, 0)]],
            [a[(1, 0)] * b_col_v[(0, 0)] + a[(1, 1)] * b_col_v[(1, 0)]]
        ])
    );

    // Transpose a row vector to a column vector
    let b_raw_v = ndarray::arr2(&[[1, 2]]);
    let c = &a.dot(&b_raw_v.t());
    assert_eq!(&a.shape(), &[2, 2]);
    assert_eq!(&b_raw_v.shape(), &[1, 2]);
    assert_eq!(&b_raw_v.t().shape(), &[2, 1]);
    assert_eq!(&c.shape(), &[2, 1]);
    assert_eq!(
        c,
        ndarray::arr2(&[
            [a[(0, 0)] * b_raw_v.t()[(0, 0)] + a[(0, 1)] * b_raw_v.t()[(1, 0)]],
            [a[(1, 0)] * b_raw_v.t()[(0, 0)] + a[(1, 1)] * b_raw_v.t()[(1, 0)]]
        ])
    );

    // Treats 1D vectors as column vectors,
    // i.e. when you multiply a 2x2 matrix by a 1D vector, the result has the shape
    //      of a 1D vector instead of a 2D column vector.
    let b = ndarray::arr1(&[1, 2]);
    let c = &a.dot(&b);
    println!("&a.dot(&b)={}", &a.dot(&b));
    assert_eq!(&a.shape(), &[2, 2]);
    assert_eq!(&b.shape(), &[2]);
    assert_eq!(&c.shape(), &[2]);
    assert_eq!(
        c,
        ndarray::arr1(&[
            a[(0, 0)] * b[0] + a[(0, 1)] * b[1],
            a[(1, 0)] * b[0] + a[(1, 1)] * b[1]
        ])
    );
}

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

// Testing Wx+b of ndarray 2D:
// Neural network forward propagation.
// To avoid miscalculation due to unintentional broadcasting of ndarray, input is given as a column vector.
#[test]
fn it_ndarray_2d_nn_forward_propagation() {
    let sigmoid = |x: f64| 1. / (1. + (-x).exp());

    let h_1 = ndarray::arr2::<f64, 2>(&[[0.1, 0.2], [0.3, 0.4]]);
    let b_1 = ndarray::arr2::<f64, 1>(&[[0.1], [0.1]]);
    let h_2 = ndarray::arr2::<f64, 2>(&[[0.5, 0.6]]);
    let b_2 = ndarray::arr2::<f64, 1>(&[[0.1]]);
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    let mini_batch_size = test_inputs.shape()[0];
    let mut h1_outputs = ndarray::Array2::zeros((2, mini_batch_size));
    let mut h2_outputs = ndarray::Array2::zeros((1, mini_batch_size));
    println!("h_1={:.4} h_1.shape()={:?}", &h_1, &h_1.shape());

    for (i, in_view) in test_inputs.rows().into_iter().enumerate() {
        println!("\ninput: {:.0}", &in_view);
        let in_col_v = in_view.insert_axis(ndarray::Axis(1));
        println!(
            "in_col_v={:.1}^t, in_col_v.shape={:?}",
            &in_col_v.t(),
            &in_col_v.shape()
        );
        let h1_out = (&h_1.dot(&in_col_v.view()) + &b_1).mapv(sigmoid);
        let mut h1_out_column = h1_outputs.column_mut(i);
        h1_out_column.assign(&h1_out.column(0).view());
        println!(
            "h1_out={:.4}^t, h1_out.shape={:?}",
            &h1_out.t(),
            &h1_out.shape()
        );

        let h2_out = (&h_2.dot(&h1_out.view()) + &b_2).mapv(sigmoid);
        println!(
            "h2_out={:.4}^t, h2_out.shape={:?}",
            &h2_out.t(),
            &h2_out.shape()
        );
        let mut output = h2_outputs.column_mut(i);
        output.assign(&h2_out.column(0).view());
    }
    println!(
        "\nh1_outputs={:.4}, shape={:?}",
        h1_outputs,
        h1_outputs.shape()
    );
    println!(
        "h1_outputs[[0, 0]],[[1, 0]]=({:.4}, {:.4})",
        h1_outputs[[0, 0]],
        h1_outputs[[1, 0]]
    );
    println!(
        "h1_outputs[[0, 1]],[[1, 1]]=({:.4}, {:.4})",
        h1_outputs[[0, 1]],
        h1_outputs[[1, 1]]
    );
    println!(
        "h1_outputs[[0, 2]],[[1, 2]]=({:.4}, {:.4})",
        h1_outputs[[0, 2]],
        h1_outputs[[1, 2]]
    );
    println!(
        "h1_outputs[[0, 3]],[[1, 3]]=({:.4}, {:.4})",
        h1_outputs[[0, 3]],
        h1_outputs[[1, 3]]
    );
    assert!((h1_outputs[[0, 2]] - 0.5498_f64).abs() < 0.001_f64);
    assert!((h1_outputs[[1, 2]] - 0.5987_f64).abs() < 0.001_f64);

    println!(
        "\nh2_outputs={:.4}, shape={:?}",
        h2_outputs,
        h2_outputs.shape()
    );
    println!("h2_outputs[[0, 0]]={:.4}", h2_outputs[[0, 0]]);
    println!("h2_outputs[[0, 1]]={:.4}", h2_outputs[[0, 1]]);
    println!(
        "h2_outputs[[0, 2]]={:.4}, error={:.5}",
        h2_outputs[[0, 2]],
        h2_outputs[[0, 2]] - 0.6757_f64
    );
    println!("h2_outputs[[0, 3]]={:.4}", h2_outputs[[0, 3]]);
    assert!((h2_outputs[[0, 2]] - 0.6757_f64).abs() <= 0.001_f64);
}
