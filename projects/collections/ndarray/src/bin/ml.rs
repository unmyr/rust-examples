// use ndarray::Array2;

fn main() {
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
    assert_eq!(
        h1_out_results_nd,
        ndarray::arr2(&[[0.1, 0.1], [2.1, 4.1], [1.1, 3.1], [3.1, 7.1]])
    );
}
