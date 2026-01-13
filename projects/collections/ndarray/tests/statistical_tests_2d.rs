#[test]
fn it_ndarray_2d_sum() {
    // Sum an empty i32 array
    let arr2_empty_i32 = ndarray::Array2::<i32>::zeros((0, 2));
    assert_eq!(arr2_empty_i32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_i32.sum(), 0);

    // Sum an empty f32 array
    let arr2_empty_f32 = ndarray::Array2::<f32>::zeros((0, 2));
    assert_eq!(arr2_empty_f32.shape(), &[0, 2]);
    assert_eq!(arr2_empty_f32.sum(), 0.);

    // Sum an i32 array
    let arr2_i32 = ndarray::arr2(&[[1, 2], [3, 4], [5, 6]]);
    assert_eq!(arr2_i32.shape(), &[3, 2]);
    assert_eq!(arr2_i32.sum(), 21);

    // Sum a f32 array
    let arr2_f32 = arr2_i32.mapv(|v| v as f32);
    assert_eq!(arr2_f32.shape(), &[3, 2]);
    assert_eq!(arr2_f32.sum(), 21.);

    // Sum an i32 array along axis 1
    let arr2_total = arr2_i32.sum_axis(ndarray::Axis(1));
    assert_eq!(arr2_total.shape(), &[3]);
    assert_eq!(arr2_total.dim(), 3);
    assert_eq!(arr2_total, ndarray::arr1(&[3, 7, 11]));

    // Sum an i32 array along axis 0
    let arr2_total = arr2_i32.sum_axis(ndarray::Axis(0));
    assert_eq!(arr2_total.shape(), &[2]);
    assert_eq!(arr2_total.dim(), 2);
    assert_eq!(arr2_total, ndarray::arr1(&[9, 12]));
}

#[test]
fn it_ndarray_2d_mean_and_var_each_rows() {
    // Calculate the average for each row
    let m2 = ndarray::array![[3., 3., 3., 3.], [2., 3., 4., 5.], [1., 2., 2., 11.]];
    println!("m2.mean={:?}, m2.var()={:?}", m2.mean(), m2.var(0.));
    let m2_mean_by_row = m2.mean_axis(ndarray::Axis(1)).unwrap();
    println!("m2_mean_by_row={:?}, ", m2_mean_by_row);
    assert_eq!(&m2_mean_by_row[0], &m2.row(0).mean().unwrap());
    assert_eq!(&m2_mean_by_row[1], &m2.row(1).mean().unwrap());
    assert_eq!(&m2_mean_by_row[2], &m2.row(2).mean().unwrap());

    let col_vec = m2_mean_by_row.insert_axis(ndarray::Axis(1));
    println!("col_vec={:?}, ", col_vec);
    assert_eq!(col_vec, ndarray::arr2(&[[3.], [3.5], [4.]]));

    let m2_var_by_row = m2.var_axis(ndarray::Axis(1), 0.);
    println!("m2_var_by_row={:?}, ", m2_var_by_row);
    assert_eq!(&m2_var_by_row[0], &m2.row(0).var(0.));
    assert_eq!(&m2_var_by_row[1], &m2.row(1).var(0.));
    assert_eq!(&m2_var_by_row[2], &m2.row(2).var(0.));
}
