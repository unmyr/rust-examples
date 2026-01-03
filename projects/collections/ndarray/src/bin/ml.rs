fn sigmoid<T: num_traits::Float>(x: T) -> T {
    T::one() / (T::one() + (-x).exp())
}

fn main() {
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
        println!("h1_out={:.4}^t, h1_out.shape={:?}", &h1_out.t(), &h1_out.shape());

        let h2_out = (&h_2.dot(&h1_out.view()) + &b_2).mapv(sigmoid);
        println!("h2_out={:.4}^t, h2_out.shape={:?}", &h2_out.t(), &h2_out.shape());
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
