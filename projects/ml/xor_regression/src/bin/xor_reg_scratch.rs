use std::time::Instant;

use clap::Parser;
use num_traits::{Float, FromPrimitive};
use plotters::prelude::*;
use rand::Rng;

#[derive(clap::Parser)]
struct Args {
    #[arg(
        short = 'n',
        long = "n-samples",
        default_value = "20000",
        action = clap::ArgAction::Set,
        help = "Max number of training samples"
    )]
    n_samples: usize,

    /// Activation function for hidden layers (identity, relu, sigmoid, tanh)
    #[arg(
        long = "hidden-activation",
        default_value = "sigmoid",
        help = "Sets the activation function for hidden layers (identity, relu, sigmoid, tanh)"
    )]
    hidden_activation: String,

    /// Activation function for output layer (identity, relu, sigmoid, tanh)
    #[arg(
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

#[derive(Debug)]
struct LayerConfig<T: Float> {
    weight: ndarray::Array2<T>,
    bias: ndarray::Array2<T>,
    act: Activation,
}

impl<T: Float> LayerConfig<T> {
    pub fn new(
        weight: ndarray::Array2<T>,
        bias: ndarray::Array2<T>,
        act: Activation,
    ) -> LayerConfig<T> {
        LayerConfig { weight, bias, act }
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
    layers: &Vec<LayerConfig<T>>,
) -> Vec<(ndarray::Array2<T>, Activation)> {
    let current_input = input.clone().into_owned();
    let mut activations = vec![(current_input, Activation::Identity)];

    for layer in layers.iter() {
        let w_out = &layer.weight.dot(&activations.last().unwrap().0.view()) + &layer.bias;
        let w_out_s = match &layer.act {
            Activation::Identity => w_out.mapv(identity),
            Activation::ReLU => w_out.mapv(relu),
            Activation::Sigmoid => w_out.mapv(sigmoid),
            Activation::Tanh => w_out.mapv(tanh),
        };
        let current_input = w_out_s;
        activations.push((current_input, layer.act.clone()));
    }

    activations
}

fn train<T: Float + std::fmt::Debug + FromPrimitive + 'static>(
    iteration: &mut usize,
    train_inputs: &ndarray::Array2<T>,
    train_answers_ref: &ndarray::Array2<T>,
    layers: &Vec<LayerConfig<T>>,
) -> (
    Vec<ndarray::Array2<T>>,
    Vec<ndarray::Array2<T>>,
    T,
    Vec<ndarray::Array1<T>>,
    Vec<ndarray::Array1<T>>,
) {
    let mini_batch_size = train_inputs.shape()[1];
    // Squared errors in the output layer
    let mut loss_terms =
        ndarray::Array2::<T>::zeros((layers.last().unwrap().weight.shape()[0], mini_batch_size));

    let mut grad_list: Vec<ndarray::Array2<T>> = Vec::new();
    let mut batch_weight_gradients: Vec<ndarray::Array2<T>> = Vec::new();
    let mut trace_outputs: Vec<ndarray::Array2<T>> = Vec::new();

    // Accumulate the gradient for each sample
    (0..layers.len()).for_each(|i| {
        grad_list.push(ndarray::Array2::zeros(layers[i].weight.dim()));
        batch_weight_gradients.push(ndarray::Array2::<T>::zeros((
            layers[i].weight.shape()[0],
            1,
        )));
        trace_outputs.push(ndarray::Array2::zeros((
            layers[i].weight.dim().0,
            mini_batch_size,
        )));
    });

    for (i, in_1d_vec_view) in train_inputs.columns().into_iter().enumerate() {
        *iteration += 1;
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward::<T>(&in_2d_col_vec.view(), layers);

        let mut cur_gradients;
        cur_gradients = &activations.last().unwrap().0 - &train_answers_ref.column(i);
        loss_terms
            .column_mut(i)
            .assign(&cur_gradients.column(0).powf(T::one() + T::one()));

        for layer_no in (0..layers.len()).rev().into_iter() {
            let a_idx = layer_no + 1;
            let act = &activations[a_idx].1;
            let l_input = &activations[a_idx - 1].0;
            let l_output = &activations[a_idx].0;

            // N-by-1 matrix representing the gradient
            let delta = match act {
                Activation::Identity => {
                    &cur_gradients * l_output.mapv(identity_derivative_from_output)
                }
                Activation::ReLU => &cur_gradients * l_output.mapv(relu_derivative_from_output),
                Activation::Sigmoid => {
                    &cur_gradients * l_output.mapv(sigmoid_derivative_from_output)
                }
                Activation::Tanh => &cur_gradients * l_output.mapv(tanh_derivative_from_output),
            };

            // Calculate gradients in a loop (using cross products)
            grad_list[layer_no].scaled_add(T::one(), &delta.dot(&l_input.t()));
            batch_weight_gradients[layer_no]
                .column_mut(0)
                .scaled_add(T::one(), &delta.column(0));

            // Trace outputs
            trace_outputs[layer_no]
                .column_mut(i)
                .assign(&l_output.clone().remove_axis(ndarray::Axis(1)));

            // Next error inputs
            cur_gradients = layers[layer_no].weight.t().dot(&delta);
        }
    }

    let trace_mean = trace_outputs
        .iter()
        .map(|v| v.mean_axis(ndarray::Axis(1)).unwrap())
        .collect::<Vec<_>>();
    let trace_var = trace_outputs
        .iter()
        .map(|v| v.var_axis(ndarray::Axis(1), T::zero()))
        .collect::<Vec<_>>();

    let loss = loss_terms.sum() / T::from(mini_batch_size).unwrap();
    (
        grad_list,
        batch_weight_gradients,
        loss,
        trace_mean,
        trace_var,
    )
}

fn plot_result(layers: &Vec<LayerConfig<f64>>) {
    let xor_continuous_pred = |x: f64, y: f64| {
        let in_2d_col_vec = ndarray::array![[x], [y]];
        let activations = forward(&in_2d_col_vec.view(), layers);
        let pred = &activations.last().unwrap().0[[0, 0]];
        return pred.clone();
    };

    let root = BitMapBackend::gif("images/xor_reg_scratch.gif", (600, 400), 100)
        .unwrap()
        .into_drawing_area();
    for pitch in 0..157 {
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Continuous XOR Approximation", ("sans-serif", 20))
            .build_cartesian_3d(-0.1..1.0, -0.1..1.0, -0.1..1.0)
            .unwrap();
        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()
            .ok();

        #[allow(unused)]
        chart.draw_series(
            SurfaceSeries::xoz(
                (-2..=20).map(|i| i as f64 * 0.05),
                (-2..=20).map(|i| i as f64 * 0.05),
                xor_continuous_pred,
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v * 1.0)).into()),
        );

        // Draw legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .ok();

        root.present().ok();
    }
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

    // Test predictions
    let n_samples = args.n_samples;

    let mut layers: Vec<LayerConfig<f64>> = Vec::new();
    let input_size: usize = 2;
    let output_size: usize = 2;
    if n_samples > 1 {
        let h = ndarray::Array2::from_shape_fn((output_size, input_size), |_| {
            rng.random_range(-0.5..0.5)
        });
        let bias =
            ndarray::Array2::from_shape_fn((output_size, 1), |_| rng.random_range(-0.5..0.5));
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    } else {
        let h = ndarray::array![[0.1, 0.2], [0.3, 0.4]];
        let bias = ndarray::array![[0.1], [0.1]];
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    }

    let input_size: usize = 2;
    let output_size: usize = 1;
    if n_samples > 1 {
        let h = ndarray::Array2::from_shape_fn((output_size, input_size), |_| {
            rng.random_range(-0.5..0.5)
        });
        let bias =
            ndarray::Array2::from_shape_fn((output_size, 1), |_| rng.random_range(-0.5..0.5));
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    } else {
        let h = ndarray::array![[0.5, 0.6]];
        let bias = ndarray::array![[0.1]];
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    }

    let learning_rate = 0.5;

    let mini_batch_size = 4;
    println!(
        "learning_rate={}, n_samples={}, mini_batch_size={}, hidden_activation={:?} output_activation={:?}",
        learning_rate, n_samples, mini_batch_size, hidden_activation, output_activation
    );

    let mut trace_means_all: Vec<Vec<_>> = Vec::new();
    let mut trace_vars_all: Vec<Vec<_>> = Vec::new();
    (0..layers.len()).for_each(|_| {
        let v: Vec<Vec<f64>> = Vec::new();
        trace_means_all.push(v);
        let v: Vec<Vec<f64>> = Vec::new();
        trace_vars_all.push(v);
    });

    let mut iteration: usize = 0;
    let mut total_trials: usize = 0;
    let t_0 = Instant::now();
    for n in 0..n_samples {
        let train_inputs: ndarray::Array2<f64>;
        if mini_batch_size == 4 {
            train_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
        } else {
            train_inputs = ndarray::Array2::<f64>::from_shape_fn((2, mini_batch_size), |_| {
                rng.random_range((0.)..(1.))
            });
        }
        let train_answers = train_inputs
            .map_axis(ndarray::Axis(0), |column| {
                xor_continuous(column[0], column[1])
            })
            .into_shape_with_order((1, mini_batch_size))
            .unwrap();

        let (grad_list, batch_weight_gradients, loss, trace_means, trace_vars) =
            train(&mut iteration, &train_inputs, &train_answers, &layers);
        (0..layers.len()).for_each(|layer_idx| {
            let (v, _offset) = &trace_means[layer_idx].clone().into_raw_vec_and_offset();
            let mut i_and_vec: Vec<f64> = vec![iteration as f64];
            i_and_vec.append(&mut v.to_vec());
            trace_means_all[layer_idx].push(i_and_vec);

            let (v, _offset) = &trace_vars[layer_idx].clone().into_raw_vec_and_offset();
            let mut i_and_vec: Vec<f64> = vec![iteration as f64];
            i_and_vec.append(&mut v.to_vec());
            trace_vars_all[layer_idx].push(i_and_vec);
        });

        // Update weight and bias
        (0..layers.len()).for_each(|i| {
            layers[i].weight.scaled_add(
                -1.,
                &(&grad_list[i] * learning_rate / (mini_batch_size as f64)),
            );
            layers[i].bias.scaled_add(
                -1.,
                &(learning_rate * &batch_weight_gradients[i] / (mini_batch_size as f64)),
            );
        });

        if n == 0 || n % 1000 == 999 {
            print!("[{:09}][{:05}]: loss={:.4}", iteration, n + 1, loss,);
            for layer_no in 0..layers.len() {
                print!(
                    ", delta[{layer_no}]^T={:.4}",
                    &batch_weight_gradients[layer_no].t()
                );
            }
            println!("");
        }

        total_trials = n;
        if loss < 0.002 {
            println!(
                "INFO: Early stopping at iteration={} due to small loss={:.4}",
                iteration, loss
            );
            break;
        }
    }

    println!("=== Results");
    let elapsed_time = t_0.elapsed();
    println!(
        "iteration={}, mini_batch_size={}, total_trials={}, learning_rate={}, hidden_activation={:?}, output_activation={:?}, elapsed time={:.2}[s] {:?}[s/mini_batch]",
        iteration,
        mini_batch_size,
        total_trials,
        learning_rate,
        hidden_activation,
        output_activation,
        elapsed_time.as_secs() as f32,
        (elapsed_time.as_secs() as f32) / (total_trials as f32)
    );

    println!("=== Trained");
    for (i, layer) in layers.iter().enumerate() {
        println!("layer[{i}]={:.4}", &layer.weight);
    }

    // Plot traces
    for (layer_idx, trace_in_layer) in trace_means_all.iter().enumerate() {
        let path = format!("images/xor_reg_scratch_{:02}_mean.png", layer_idx);
        let root_area = BitMapBackend::new(&path, (600, 400)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        // Draw chart for each weight
        // Number of weights
        let n_weights = trace_in_layer[0].len() - 1;
        let mut chart = ChartBuilder::on(&root_area)
            .caption(
                format!(
                    "The average output of a mini-batch: (Layer:{:0}; {:?})",
                    layer_idx, n_weights
                ),
                ("sans-serif", 20),
            )
            .margin(10)
            .margin_right(30)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d((1.)..(iteration as f64), -0.1..1.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{:.0}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("n samples")
            .y_desc("weights")
            .draw()
            .ok();
        let series_len = trace_in_layer[0].len();
        (1..series_len).for_each(|w_idx| {
            let color = Palette99::pick(w_idx).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    trace_in_layer.iter().map(|v| (v[0] as f64, v[w_idx])),
                    color.stroke_width(2),
                ))
                .unwrap()
                .label(format!("w{:?}", w_idx))
                .legend(move |(x, y)| Rectangle::new([(x, y), (x + 10, y + 1)], color.filled()));
        });

        // Draw legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .ok();

        println!("Saved the figure to: {}", path);
    }

    // Plot traces
    for (layer_idx, trace_in_layer) in trace_vars_all.iter().enumerate() {
        let path = format!("images/xor_reg_scratch_{:02}_var.png", layer_idx);
        let root_area = BitMapBackend::new(&path, (600, 400)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        // Draw chart for each weight
        let n_weights = trace_in_layer[0].len() - 1;
        let mut chart = ChartBuilder::on(&root_area)
            .caption(
                format!(
                    "The variance output of a mini-batch: (Layer:{:0}; {:?})",
                    layer_idx, n_weights
                ),
                ("sans-serif", 20),
            )
            .margin(10)
            .margin_right(30)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d((1.)..(iteration as f64 + 10_f64), -0.1..1.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{:.0}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("n samples")
            .y_desc("weights")
            .draw()
            .ok();
        let series_len = trace_in_layer[0].len();
        (1..series_len).for_each(|w_idx| {
            let color = Palette99::pick(w_idx).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    trace_in_layer.iter().map(|v| (v[0] as f64, v[w_idx])),
                    color.stroke_width(2),
                ))
                .unwrap()
                .label(format!("w{:?}", w_idx))
                .legend(move |(x, y)| Rectangle::new([(x, y), (x + 10, y + 1)], color.filled()));
        });

        // Draw legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .ok();

        println!("Saved the figure to: {}", path);
    }

    println!("\n== XOR Predictions ==");
    let mut correct_counts = 0;
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
    let test_batch_size = test_inputs.shape()[1];
    let test_answers = test_inputs
        .map_axis(ndarray::Axis(0), |column| {
            xor_continuous(column[0], column[1])
        })
        .into_shape_with_order((1, test_batch_size))
        .unwrap();
    for (i, in_1d_vec_view) in test_inputs.columns().into_iter().enumerate() {
        let (x1, x2) = (in_1d_vec_view[0], in_1d_vec_view[1]);
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward(&in_2d_col_vec.view(), &layers);
        let answer = test_answers[[0, i]];
        let ans11 = ndarray::arr2(&[[answer]]);
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
        (correct_counts as f64 / (test_batch_size as f64)) * 100.0
    );

    plot_result(&layers);
}
