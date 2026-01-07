use clap::Parser;
use num_traits::Float;
use rand::Rng;
use std::time::Instant;

#[derive(clap::Parser)]
struct Args {
    /// Activation function for hidden layers (identity, relu, sigmoid, tanh)
    #[clap(
        long = "hidden-activation",
        default_value = "sigmoid",
        help = "Sets the activation function for hidden layers (identity, relu, sigmoid, tanh)"
    )]
    hidden_activation: String,

    /// Activation function for output layer (identity, relu, sigmoid, tanh)
    #[clap(
        long = "output-activation",
        default_value = "sigmoid",
        help = "Sets the activation function for output layer (identity, relu, sigmoid, tanh)"
    )]
    output_activation: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
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
    input: &ndarray::ArrayView2<T>,
    layer1: &(ndarray::Array2<T>, ndarray::Array2<T>, Activation),
    layer2: &(ndarray::Array2<T>, ndarray::Array2<T>, Activation),
) -> Vec<(ndarray::Array2<T>, Activation)> {
    let current_input = input.clone().into_owned();
    let mut activations = vec![(current_input, Activation::Identity)];

    for layer in [layer1, layer2] {
        let weight = &layer.0;
        let bias = &layer.1;
        let act = &layer.2;
        let w_out = weight.dot(&activations.last().unwrap().0.view()) + bias;
        let w_out_s = match act {
            Activation::Identity => w_out.mapv(identity),
            Activation::ReLU => w_out.mapv(relu),
            Activation::Sigmoid => w_out.mapv(sigmoid),
            Activation::Tanh => w_out.mapv(tanh),
        };
        let current_input = w_out_s;
        activations.push((current_input, act.clone()));
    }

    activations
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Retrieve the value of --hidden-activation argument
    let hidden_activation = match String::from(&args.hidden_activation).as_str() {
        "identity" => Activation::Identity,
        "relu" => Activation::ReLU,
        "sigmoid" => Activation::Sigmoid,
        "tanh" => Activation::Tanh,
        _ => {
            // Handle unknown activation function
            // Using default activation function instead.
            eprintln!(
                "WARNING: Unknown activation function specified; using sigmoid function instead: {}",
                &args.hidden_activation
            );
            Activation::Sigmoid
        }
    };

    // Retrieve the value of --output-activation argument
    let output_activation = match String::from(&args.output_activation).as_str() {
        "identity" => Activation::Identity,
        "relu" => Activation::ReLU,
        "sigmoid" => Activation::Sigmoid,
        "tanh" => Activation::Tanh,
        _ => {
            // Handle unknown activation function
            // Using default activation function instead.
            eprintln!(
                "WARNING: Unknown activation function specified; using sigmoid function instead: {}",
                &args.output_activation
            );
            Activation::Sigmoid
        }
    };

    let mut rng = rand::rng();
    let init_random_value = true;

    // Test predictions
    let n_samples = 20000;
    let h1;
    let h2;
    let bias1;
    let bias2;
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
    let mut layers: Vec<(ndarray::Array2<f64>, ndarray::Array2<f64>, Activation)> = Vec::new();
    layers.push((h1, bias1, hidden_activation.clone()));
    layers.push((h2, bias2, output_activation.clone()));

    let learning_rate = 0.5;

    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
    let test_answers = ndarray::arr2(&[[0., 1., 1., 0.]]);
    let mini_batch_size = test_inputs.shape()[0];
    println!(
        "learning_rate={}, n_samples={}, mini_batch_size={}, hidden_activation={:?} output_activation={:?}",
        learning_rate, n_samples, mini_batch_size, hidden_activation, output_activation
    );
    let t_0 = Instant::now();
    for n in 0..n_samples {
        let mut h2s_outputs = ndarray::Array2::<f64>::zeros((1, mini_batch_size));

        // Accumulate the gradient for each sample
        let mut grad_list: Vec<ndarray::Array2<f64>> = Vec::new();
        let mut delta_total_list: Vec<ndarray::Array2<f64>> = Vec::new();
        (0..layers.len()).for_each(|i| {
            grad_list.push(ndarray::Array2::zeros(layers[i].0.dim()));
            delta_total_list.push(ndarray::Array2::<f64>::zeros((layers[i].0.shape()[0], 1)));
        });

        for (i, in_view) in test_inputs.rows().into_iter().enumerate() {
            let in_col_v = in_view.insert_axis(ndarray::Axis(1));
            let (x1, x2) = (in_col_v[[0, 0]], in_col_v[[1, 0]]);
            // let (x1, x2) = (rng.random_range(0.0..=1.0), rng.random_range(0.0..=1.0));
            let activations = forward::<f64>(&in_col_v.view(), &layers[0], &layers[1]);

            let mut h2s_output_column = h2s_outputs.column_mut(i);
            h2s_output_column.assign(&activations[2].0.column(0).view());

            // Output layer 1
            let layer_no = 2;
            let y_train = ndarray::arr2::<f64, 1>(&[[xor_continuous(x1, x2)]]);
            let output_error = &activations[layer_no].0 - &y_train;
            let act = &activations[layer_no].1;
            let delta_h2 = match act {
                Activation::Identity => {
                    &output_error
                        * &activations[layer_no]
                            .0
                            .mapv(identity_derivative_from_output)
                }
                _ => &output_error * &activations[layer_no].0.mapv(sigmoid_derivative_from_output),
            };
            // Calculate gradients in a loop (using cross products)
            grad_list[layer_no - 1] += &delta_h2.dot(&activations[layer_no - 1].0.t());

            let mut delta_h2_total_work = delta_total_list[1].column_mut(0);
            let delta_sum_h2 = &delta_h2_total_work.view() + &delta_h2.column(0).view();
            delta_h2_total_work.assign(&delta_sum_h2.view());

            // Hidden layer 1
            let layer_no = 1;
            let act = &activations[layer_no].1;
            let delta_h1 = match act {
                Activation::Identity => {
                    &layers[layer_no].0.t().dot(&delta_h2)
                        * &activations[layer_no].0.mapv(sigmoid_derivative_from_output)
                }
                Activation::ReLU => {
                    &layers[layer_no].0.t().dot(&delta_h2)
                        * &activations[layer_no].0.mapv(relu_derivative_from_output)
                }
                Activation::Sigmoid => {
                    &layers[layer_no].0.t().dot(&delta_h2)
                        * &activations[layer_no].0.mapv(sigmoid_derivative_from_output)
                }
                Activation::Tanh => {
                    &layers[layer_no].0.t().dot(&delta_h2)
                        * &activations[layer_no].0.mapv(tanh_derivative_from_output)
                }
            };
            grad_list[layer_no - 1] += &delta_h1.dot(&activations[layer_no - 1].0.t());

            let mut delta_h1_total_work = delta_total_list[0].column_mut(0);
            let delta_sum_h1 = &delta_h1_total_work.view() + &delta_h1.column(0).view();
            delta_h1_total_work.assign(&delta_sum_h1.view());
        }
        // Update weight and bias
        (0..layers.len()).for_each(|i| {
            layers[i].0 = &layers[i].0 - &grad_list[i] * learning_rate / (mini_batch_size as f64);
            layers[i].1 =
                &layers[i].1 - learning_rate * &delta_total_list[i] / (mini_batch_size as f64);
        });

        let loss = (&h2s_outputs - &test_answers).powf(2.).sum() / (mini_batch_size as f64);
        if n == 0 || n % 1000 == 999 {
            println!(
                "[{:05}]: loss={:.4} delta_h1_total^T={:.4}, delta_h2_total^T={:.4}",
                n + 1,
                loss,
                &delta_total_list[0].t(),
                &delta_total_list[1].t()
            );
        }
    }

    println!("== Results ==");
    let elapsed_time = t_0.elapsed();
    println!(
        "learning_rate={}, n_samples={}, mini_batch_size={}, hidden_activation={:?}, output_activation={:?}, elapsed time={:.2}[s] {:?}[s/sample]",
        learning_rate,
        n_samples,
        mini_batch_size,
        hidden_activation,
        output_activation,
        elapsed_time.as_secs() as f32,
        (elapsed_time.as_secs() as f32) / (n_samples as f32)
    );
    println!("== Trained ===");
    println!("h1={:.4}", &layers[0].0);
    println!("h2={:.4}", &layers[1].0);

    println!("\n== XOR Predictions ==");
    let mut correct_counts = 0;
    for in_view in test_inputs.rows() {
        let in_col_v = in_view.insert_axis(ndarray::Axis(1));
        let (x1, x2) = (in_col_v[[0, 0]], in_col_v[[1, 0]]);
        let activations = forward(&in_col_v.view(), &layers[0], &layers[1]);

        let ans11 = ndarray::arr2(&[[xor_continuous(x1, x2)]]);
        let loss = (&ans11 - &activations[2].0).powf(2.).sum() / 2.;
        if loss < 0.05 {
            correct_counts += 1;
        }
        println!(
            "Input: [{:?}] => Predicted: {:.2}, answer: {:.0}, loss: {:.2}",
            [x1, x2],
            &activations[2].0[[0, 0]],
            ans11[[0, 0]],
            loss
        );
    }
    println!(
        "Accuracy: {:.2}%",
        (correct_counts as f64 / (test_inputs.shape()[0]) as f64) * 100.0
    );
}
