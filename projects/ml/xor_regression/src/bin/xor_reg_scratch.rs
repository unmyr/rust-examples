use clap::Parser;
use num_traits::Float;
use rand::Rng;
use std::time::Instant;

#[derive(clap::Parser)]
struct Args {
    /// Activation function to use (identity, relu, sigmoid, tanh)
    #[clap(
        long = "activation",
        short = 'a',
        default_value = "sigmoid",
        help = "Sets the activation function (identity, relu, sigmoid, tanh)"
    )]
    activation: String,
}

#[allow(unused)]
#[derive(Debug)]
enum Activation {
    Identity,
    ReLU,
    Sigmoid,
    Tanh,
}

impl std::fmt::Display for Activation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Activation::Identity => write!(f, "identity"),
            Activation::Sigmoid => write!(f, "sigmoid"),
            Activation::Tanh => write!(f, "tanh"),
            Activation::ReLU => write!(f, "ReLU"),
        }
    }
}

// Identity function (does nothing)
fn identity<T>(x: T) -> T {
    x
}

// The derivative of the identity function is always 1
fn identity_derivative_from_output<T: Float>(_: T) -> T {
    T::one()
}

fn relu<T: Float>(x: T) -> T {
    if x > T::zero() {
        x
    } else {
        // T::zero()
        // Leaky ReLU
        T::from(0.01).unwrap() * x
    }
}

fn relu_derivative_from_output<T: Float>(s: T) -> T {
    if s > T::zero() {
        T::one()
    } else {
        // T::zero()
        // Leaky ReLU
        T::from(0.01).unwrap()
    }
}

// The domain is [0, 1]
fn sigmoid<T: Float>(x: T) -> T {
    T::one() / (T::one() + (-x).exp())
}

// Calculated from the output of the activation function
fn sigmoid_derivative_from_output<T: Float>(s: T) -> T {
    s * (T::one() - s)
}

// The domain is [-1,1]
fn tanh<T: Float>(x: T) -> T {
    x.tanh()
}

// Calculated from the output of the activation function
fn tanh_derivative_from_output<T: Float>(t: T) -> T {
    T::one() - t * t
}

fn xor_continuous<T: Float>(x1: T, x2: T) -> T {
    x1 + x2 - T::from(2.0).unwrap() * x1 * x2
}

fn forward<T: Float + 'static>(
    function: &Activation,
    input: &ndarray::ArrayView2<T>,
    layer1: (&ndarray::Array2<T>, &ndarray::Array2<T>),
    layer2: (&ndarray::Array2<T>, &ndarray::Array2<T>),
) -> (
    ndarray::Array2<T>,
    ndarray::Array2<T>,
    Vec<ndarray::Array2<T>>,
) {
    let (h1, bias1) = layer1;
    let (h2, bias2) = layer2;

    let current_input = input.clone().into_owned();
    let mut activations = vec![current_input];
    let h1_out = h1.dot(input) + bias1;
    let h1_out_s = match function {
        Activation::Identity => h1_out.mapv(sigmoid),
        Activation::ReLU => h1_out.mapv(relu),
        Activation::Sigmoid => h1_out.mapv(sigmoid),
        Activation::Tanh => h1_out.mapv(tanh),
    };
    let current_input = h1_out_s.clone();
    activations.push(current_input);

    let h2_out = h2.dot(&h1_out_s) + bias2;
    let h2_out_s = match function {
        Activation::Identity => h2_out.mapv(identity),
        Activation::ReLU => h2_out.mapv(sigmoid),
        Activation::Sigmoid => h2_out.mapv(sigmoid),
        Activation::Tanh => h2_out.mapv(sigmoid),
    };
    let current_input = h2_out_s.clone();
    activations.push(current_input);

    (h1_out, h2_out, activations)
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Retrieve the value of --activation argument
    let activation = match String::from(&args.activation).as_str() {
        "identity" => Activation::Identity,
        "relu" => Activation::ReLU,
        "sigmoid" => Activation::Sigmoid,
        "tanh" => Activation::Tanh,
        _ => {
            // Handle unknown activation function
            // Using default activation function instead.
            eprintln!(
                "WARNING: Unknown activation function specified; using sigmoid function instead: {}",
                &args.activation
            );
            Activation::Sigmoid
        }
    };
    let mut rng = rand::rng();
    let init_random_value = true;

    // Test predictions
    let n_samples = 20000;
    let mut h1;
    let mut h2;
    let mut bias1;
    let mut bias2;
    if init_random_value {
        h1 = ndarray::Array2::from_shape_fn((2, 2), |_| rng.random_range(-0.5..0.5));
        h2 = ndarray::Array2::from_shape_fn((1, 2), |_| rng.random_range(-0.5..0.5));
        bias1 = ndarray::Array2::from_shape_fn((2, 1), |_| rng.random_range(-0.5..0.5));
        bias2 = ndarray::Array2::from_shape_fn((1, 1), |_| rng.random_range(-0.5..0.5));
    } else {
        h1 = ndarray::arr2::<f64, 2>(&[[0.1, 0.2], [0.3, 0.4]]);
        h2 = ndarray::arr2::<f64, 2>(&[[0.5, 0.6]]);
        bias1 = ndarray::arr2::<f64, 1>(&[[0.1], [0.1]]);
        bias2 = ndarray::arr2::<f64, 1>(&[[0.1]]);
    }

    let learning_rate = 0.5;

    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    let test_answers = ndarray::arr2(&[[0., 1., 1., 0.]]);
    let mini_batch_size = test_inputs.shape()[0];
    println!(
        "learning_rate={}, n_samples={}, mini_batch_size={}, activation={:?}",
        learning_rate, n_samples, mini_batch_size, activation
    );
    let t_0 = Instant::now();
    for n in 0..n_samples {
        let mut h2s_outputs = ndarray::Array2::<f64>::zeros((1, mini_batch_size));

        // Accumulate the gradient for each sample
        let mut grad_h2 = ndarray::Array2::zeros(h2.dim());
        let mut grad_h1 = ndarray::Array2::zeros(h1.dim());
        let mut delta_h1_total = ndarray::Array2::<f64>::zeros((2, 1));
        let mut delta_h2_total = ndarray::Array2::<f64>::zeros((1, 1));
        for (i, in_view) in test_inputs.rows().into_iter().enumerate() {
            let in_col_v = in_view.insert_axis(ndarray::Axis(1));
            let (x1, x2) = (in_col_v[[0, 0]], in_col_v[[1, 0]]);
            // let (x1, x2) = (rng.random_range(0.0..=1.0), rng.random_range(0.0..=1.0));
            let y_train = ndarray::arr2::<f64, 1>(&[[xor_continuous(x1, x2)]]);
            let (_h1_out, _h2_out, activations) =
                forward::<f64>(&activation, &in_col_v.view(), (&h1, &bias1), (&h2, &bias2));
            let h2_out_s = activations[2].clone();
            let mut h2s_output_column = h2s_outputs.column_mut(i);
            h2s_output_column.assign(&h2_out_s.column(0).view());

            let output_error = &h2_out_s - &y_train;
            let delta_h2 = match activation {
                Activation::Identity => {
                    &output_error * &h2_out_s.mapv(identity_derivative_from_output)
                }
                _ => &output_error * &h2_out_s.mapv(sigmoid_derivative_from_output),
            };
            let mut delta_h2_total_work = delta_h2_total.column_mut(0);
            let delta_sum_h2 = &delta_h2_total_work.view() + &delta_h2.column(0).view();
            delta_h2_total_work.assign(&delta_sum_h2.view());

            let h1_out_s = activations[1].clone();
            let delta_h1 = match activation {
                Activation::Identity => {
                    h2.t().dot(&delta_h2) * h1_out_s.mapv(sigmoid_derivative_from_output)
                }
                Activation::ReLU => {
                    h2.t().dot(&delta_h2) * h1_out_s.mapv(relu_derivative_from_output)
                }
                Activation::Sigmoid => {
                    h2.t().dot(&delta_h2) * h1_out_s.mapv(sigmoid_derivative_from_output)
                }
                Activation::Tanh => {
                    h2.t().dot(&delta_h2) * h1_out_s.mapv(tanh_derivative_from_output)
                }
            };

            let in_col_v = &activations[0];

            // Calculate gradients in a loop (using cross products)
            grad_h2 += &delta_h2.dot(&h1_out_s.t());
            grad_h1 += &delta_h1.dot(&in_col_v.t());

            let mut delta_h1_total_work = delta_h1_total.column_mut(0);
            let delta_sum_h1 = &delta_h1_total_work.view() + &delta_h1.column(0).view();
            delta_h1_total_work.assign(&delta_sum_h1.view());
        }
        h2 = &h2 - &grad_h2 * learning_rate / (mini_batch_size as f64);
        bias2 = &bias2 - learning_rate * &delta_h2_total / (mini_batch_size as f64);

        h1 = &h1 - &grad_h1 * learning_rate / (mini_batch_size as f64);
        bias1 = &bias1 - learning_rate * &delta_h1_total / (mini_batch_size as f64);

        let loss = (&h2s_outputs - &test_answers).powf(2.).sum() / (mini_batch_size as f64);
        if n == 0 || n % 1000 == 999 {
            println!(
                "[{:05}]: loss={:.4} delta_h1_total^T={:.4}, delta_h2_total={:.4}",
                n + 1,
                loss,
                delta_h1_total.t(),
                delta_h2_total
            );
        }
    }

    println!("== Results ==");
    let elapsed_time = t_0.elapsed();
    println!(
        "learning_rate={}, n_samples={}, mini_batch_size={}, activation={:?}, elapsed time={:.2}[s] {:?}[s/sample]",
        learning_rate,
        n_samples,
        mini_batch_size,
        activation,
        elapsed_time.as_secs() as f32,
        (elapsed_time.as_secs() as f32) / (n_samples as f32)
    );
    println!("== Trained ===");
    println!("h1={h1:.4}");
    println!("h2={h2:.4}");

    println!("\n== XOR Predictions ==");
    let mut correct_counts = 0;
    for in_view in test_inputs.rows() {
        let in_col_v = in_view.insert_axis(ndarray::Axis(1));
        let (x1, x2) = (in_col_v[[0, 0]], in_col_v[[1, 0]]);
        let (_h1_out, _h2_out, activations) =
            forward(&activation, &in_col_v.view(), (&h1, &bias1), (&h2, &bias2));
        let h2_out_s_ref = &activations[2];

        let ans11 = ndarray::arr2(&[[xor_continuous(x1, x2)]]);
        let loss = (&ans11 - h2_out_s_ref).powf(2.).sum() / 2.;
        if loss < 0.05 {
            correct_counts += 1;
        }
        println!(
            "Input: [{:?}] => Predicted: {:.2}, answer: {:.0}, loss: {:.2}",
            [x1, x2],
            h2_out_s_ref[[0, 0]],
            ans11[[0, 0]],
            loss
        );
    }
    println!(
        "Accuracy: {:.2}%",
        (correct_counts as f64 / (test_inputs.shape()[0]) as f64) * 100.0
    );
}
