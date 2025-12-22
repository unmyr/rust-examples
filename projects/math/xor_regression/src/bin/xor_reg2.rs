fn xor_continuous(x1: f64, x2: f64) -> f64 {
    (x1.asin() + x2.asin()).sin()
}

fn main() {
    // Test predictions
    let test_inputs = ndarray::array![
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0]
    ];

    println!("== XOR Predictions ==");
    for input in test_inputs.rows() {
        let x1 = input[0];
        let x2 = input[1];
        let pred = xor_continuous(x1, x2);
        println!("Input: [{}, {}] => Predicted: {:.3}", x1, x2, pred);
    }
}
