use std::time::Instant;

use clap::Parser;
use num_traits::{Float, FromPrimitive};
use plotters::prelude::*;
use rand::Rng;
use tracing::info;
use tracing_subscriber::{self};

// Command-line arguments structure
#[derive(clap::Parser)]
struct Args {
    #[arg(
        long = "batch-size",
        default_value = "4",
        action = clap::ArgAction::Set,
        help = "Number of epochs to train"
    )]
    batch_size: usize,

    #[arg(
        short = 'n',
        long = "max-epoch",
        default_value = "20000",
        action = clap::ArgAction::Set,
        help = "Max number of training samples"
    )]
    max_epoch: usize,

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

// Activation functions supported
#[derive(Debug, Clone, Copy)]
enum Activation {
    Identity,
    ReLU,
    Sigmoid,
    Tanh,
}

// Implement Display trait for Activation enum
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

// Configuration of a single layer in the neural network
struct LayerConfig<F: Float> {
    weight: ndarray::Array2<F>,
    bias: ndarray::Array2<F>,
    act: Activation,
}

// Implement constructor for LayerConfig
impl<F: Float> LayerConfig<F> {
    pub fn new(
        weight: ndarray::Array2<F>,
        bias: ndarray::Array2<F>,
        act: Activation,
    ) -> LayerConfig<F> {
        LayerConfig { weight, bias, act }
    }
}

impl<F: std::fmt::Debug + Float> std::fmt::Debug for LayerConfig<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{weight: {}, bias: {}, activation: {:?}}}",
            format!("{:.4?}", &self.weight.view()).replace("\n", ""),
            format!("{:.4?}", &self.bias.view()).replace("\n", ""),
            self.act
        )
    }
}

// Record of trace information during training
#[derive(Debug)]
struct TraceRecord<F: Float> {
    iteration: usize,
    mean: Vec<ndarray::Array1<F>>,
    variance: Vec<ndarray::Array1<F>>,
    cosine_similarity_row: Vec<F>,
    cosine_similarity_col: Vec<F>,
}

// Implement constructor for TraceRecord
impl<F: Float> TraceRecord<F> {
    pub fn new(
        iteration: usize,
        mean: Vec<ndarray::Array1<F>>,
        variance: Vec<ndarray::Array1<F>>,
        cosine_similarity_row: Vec<F>,
        cosine_similarity_col: Vec<F>,
    ) -> TraceRecord<F> {
        TraceRecord {
            iteration: iteration,
            mean: mean,
            variance: variance,
            cosine_similarity_row: cosine_similarity_row,
            cosine_similarity_col: cosine_similarity_col,
        }
    }
}

// Structure to hold training results for serialization
#[derive(serde::Serialize, serde::Deserialize)]
struct TrainResults<F: Float> {
    accuracy: F,
    rmse: F,
    layers: usize,
    learning_rate: F,
    mini_batch_size: usize,
    last_epoch: usize,
    hidden_activation: String,
    output_activation: String,
}

impl<F: Float> TrainResults<F> {
    pub fn new(
        accuracy: F,
        rmse: F,
        layers: usize,
        learning_rate: F,
        mini_batch_size: usize,
        last_epoch: usize,
        hidden_activation: &String,
        output_activation: &String,
    ) -> TrainResults<F> {
        TrainResults {
            accuracy,
            rmse,
            layers,
            learning_rate,
            mini_batch_size,
            last_epoch,
            hidden_activation: hidden_activation.clone(),
            output_activation: output_activation.clone(),
        }
    }
}

// Identity function (does nothing)
fn identity<F>(x: F) -> F {
    x
}

// The derivative of the identity function is always 1
fn identity_derivative_from_output<F: Float>(_: F) -> F {
    F::one()
}

// ReLU function
fn relu<F: Float>(x: F) -> F {
    if x > F::zero() {
        x
    } else {
        // F::zero()
        // Leaky ReLU
        F::from(0.01).unwrap() * x
    }
}

// Calculated from the output of the activation function
fn relu_derivative_from_output<F: Float>(s: F) -> F {
    if s > F::zero() {
        F::one()
    } else {
        // F::zero()
        // Leaky ReLU
        F::from(0.01).unwrap()
    }
}

// The domain is [0, 1]
fn sigmoid<F: Float>(x: F) -> F {
    F::one() / (F::one() + (-x).exp())
}

// Calculated from the output of the activation function
fn sigmoid_derivative_from_output<F: Float>(s: F) -> F {
    s * (F::one() - s)
}

// The domain is [-1,1]
fn tanh<F: Float>(x: F) -> F {
    x.tanh()
}

// Calculated from the output of the activation function
fn tanh_derivative_from_output<F: Float>(t: F) -> F {
    F::one() - t * t
}

// Continuous XOR function
fn xor_continuous<F: Float>(x1: F, x2: F) -> F {
    x1 + x2 - F::from(2.0).unwrap() * x1 * x2
}

fn argmax<T: PartialOrd>(v: &[T]) -> Option<usize> {
    v.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
}

// Calculate cosine similarity between row vector in 2D array
fn cosine_similarity_vec<F: num_traits::Float>(m: &ndarray::ArrayView2<F>) -> Vec<F> {
    if m.dim().1 == 1 {
        return vec![F::one()];
    }
    let mut similarities = Vec::new();
    for i in 0..m.nrows() {
        for j in (i + 1)..m.nrows() {
            let row_i = m.row(i);
            let row_j = m.row(j);
            let mut vec_squared = ndarray::Array1::<F>::zeros(m.dim().1);
            // let dot_product = v1.dot(&v2.t());
            ndarray::Zip::from(&mut vec_squared)
                .and(row_i)
                .and(row_j)
                .for_each(|r, &i_k, &j_k| *r = i_k * j_k);
            let dot_product = vec_squared.sum();
            let norm_row_i = row_i.mapv(|v| v * v).sum().sqrt();
            let norm_row_j = row_j.mapv(|v| v * v).sum().sqrt();
            similarities.push(dot_product / (norm_row_i * norm_row_j));
        }
    }
    similarities
}

// Forward propagation
fn forward<F: Float + 'static>(
    input: &ndarray::ArrayView2<F>,
    layers: &Vec<LayerConfig<F>>,
) -> Vec<(ndarray::Array2<F>, Activation)> {
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

// Training function
fn train<F: Float + std::fmt::Debug + FromPrimitive + 'static>(
    epoch: usize,
    train_inputs: &ndarray::Array2<F>,
    train_answers_ref: &ndarray::Array2<F>,
    layers: &Vec<LayerConfig<F>>,
) -> (
    Vec<ndarray::Array2<F>>,
    Vec<ndarray::Array2<F>>,
    F,
    TraceRecord<F>,
) {
    let mini_batch_size = train_inputs.shape()[1];
    // Squared errors in the output layer
    let mut loss_terms =
        ndarray::Array2::<F>::zeros((layers.last().unwrap().weight.shape()[0], mini_batch_size));

    let mut grad_list: Vec<ndarray::Array2<F>> = Vec::new();
    let mut batch_weight_gradients: Vec<ndarray::Array2<F>> = Vec::new();
    let mut trace_outputs: Vec<ndarray::Array2<F>> = Vec::new();

    // Accumulate the gradient for each sample
    (0..layers.len()).for_each(|i| {
        grad_list.push(ndarray::Array2::zeros(layers[i].weight.dim()));
        batch_weight_gradients.push(ndarray::Array2::<F>::zeros((
            layers[i].weight.shape()[0],
            1,
        )));
        trace_outputs.push(ndarray::Array2::zeros((
            layers[i].weight.dim().0,
            mini_batch_size,
        )));
    });

    for (i, in_1d_vec_view) in train_inputs.columns().into_iter().enumerate() {
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward::<F>(&in_2d_col_vec.view(), layers);

        let mut cur_gradients;
        cur_gradients = &activations.last().unwrap().0 - &train_answers_ref.column(i);
        loss_terms
            .column_mut(i)
            .assign(&cur_gradients.column(0).powf(F::one() + F::one()));

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
            grad_list[layer_no].scaled_add(F::one(), &delta.dot(&l_input.t()));
            batch_weight_gradients[layer_no]
                .column_mut(0)
                .scaled_add(F::one(), &delta.column(0));

            // Trace outputs
            trace_outputs[layer_no]
                .column_mut(i)
                .assign(&l_output.clone().remove_axis(ndarray::Axis(1)));

            // Next error inputs
            cur_gradients = layers[layer_no].weight.t().dot(&delta);
        }
    }

    // Calculate trace statistics
    // Mean and variance
    let trace_mean = trace_outputs
        .iter()
        .map(|v| v.mean_axis(ndarray::Axis(1)).unwrap())
        .collect::<Vec<_>>();

    let trace_var = trace_outputs
        .iter()
        .map(|v| v.var_axis(ndarray::Axis(1), F::zero()))
        .collect::<Vec<_>>();

    // Calculate cosine similarities
    let mut trace_row_sim = Vec::<F>::new();
    let mut trace_col_sim = Vec::<F>::new();
    for layer_no in (0..layers.len()).into_iter() {
        if &layers[layer_no].weight.dim().0 == &1_usize {
            trace_row_sim.push(F::zero());
            trace_col_sim.push(F::zero());
            continue;
        }

        let sim_row_vec = cosine_similarity_vec(&layers[layer_no].weight.view());
        let max_row_idx = sim_row_vec
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        trace_row_sim.push(sim_row_vec[max_row_idx]);

        let sim_col_vec = cosine_similarity_vec(&layers[layer_no].weight.t());
        let max_col_idx = sim_col_vec
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        trace_col_sim.push(sim_col_vec[max_col_idx]);
    }

    let trace = TraceRecord::new(epoch, trace_mean, trace_var, trace_row_sim, trace_col_sim);
    let loss = loss_terms.sum() / F::from(mini_batch_size).unwrap();
    (grad_list, batch_weight_gradients, loss, trace)
}

// Plot the result of the XOR regression
fn plot_result(layers: &Vec<LayerConfig<f64>>, base_name: String) {
    let xor_continuous_pred = |x: f64, y: f64| {
        let in_2d_col_vec = ndarray::array![[x], [y]];
        let activations = forward(&in_2d_col_vec.view(), layers);
        let pred = &activations.last().unwrap().0[[0, 0]];
        return pred.clone();
    };

    let image_path = format!("images/{base_name}.gif");
    let root = BitMapBackend::gif(&image_path, (600, 400), 100)
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
    info!("Saved the figure to: {}", image_path);
}

// Main function
fn main() {
    // Get program name
    let args: Vec<String> = std::env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => {
            let cmd_path = std::path::Path::new(arg0);
            let program_name = cmd_path.file_name();
            let program_name = program_name.unwrap().to_str().unwrap();
            program_name
        }
        None => "xor_reg_scratch",
    };

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

    // Initialize the JSON logger
    tracing_subscriber::fmt()
        .json() // Enable JSON formatting
        .init();

    let mut rng = rand::rng();

    let mini_batch_size = args.batch_size;
    let max_epoch = args.max_epoch;

    let mut layers: Vec<LayerConfig<f64>> = Vec::new();
    let mut trace_biases: Vec<Vec<(usize, Vec<f64>)>> = Vec::new();
    let mut cosine_similarities: Vec<f64> = Vec::new();

    let input_size: usize = 2;
    let output_size: usize = 2;
    if max_epoch > 1 {
        let h = ndarray::Array2::from_shape_fn((output_size, input_size), |_| {
            rng.random_range(-0.5..0.5)
        });
        let cosine_similarity = (&h.row(0) * &h.row(1)).sum()
            / (&h.row(0).mapv(|v| v * v).sum().powf(0.5)
                * &h.row(1).mapv(|v| v * v).sum().powf(0.5));
        cosine_similarities.push(cosine_similarity);
        info!(
            event = "Verifying orthogonality of weight matrices for training",
            layer_no = layers.len(),
            cosine_similarity = cosine_similarity
        );
        let bias = ndarray::Array2::from_shape_fn((output_size, 1), |_| 0.5);
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    } else {
        let h = ndarray::array![[0.1, 0.2], [0.3, 0.4]];
        let bias = ndarray::array![[0.1], [0.1]];
        let layer = LayerConfig::<f64>::new(h, bias, hidden_activation.clone());
        layers.push(layer);
    }
    let current_layer_no = layers.len() - 1;
    info!(
        event = "Initial layer configuration",
        layer_no = current_layer_no,
        weight = &layers[current_layer_no]
            .weight
            .to_string()
            .replace("\n", ""),
        bias = &layers[current_layer_no].bias.to_string().replace("\n", ""),
        activation = format!("{:?}", layers[current_layer_no].act),
        cosine_similarity = argmax(&cosine_similarity_vec(
            &layers[current_layer_no].weight.view()
        ))
        .unwrap()
    );

    let input_size: usize = output_size;
    let output_size: usize = 1;
    if max_epoch > 1 {
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
    let current_layer_no = layers.len() - 1;
    info!(
        event = "Initial layer configuration",
        layer_no = current_layer_no,
        weight = &layers[current_layer_no]
            .weight
            .to_string()
            .replace("\n", ""),
        bias = &layers[current_layer_no].bias.to_string().replace("\n", ""),
        activation = format!("{:?}", layers[current_layer_no].act)
    );

    (0..layers.len()).for_each(|_| {
        trace_biases.push(Vec::new());
    });

    let learning_rate = 0.5;

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
        .into_shape_with_order((1, train_inputs.dim().1))
        .unwrap();

    info!(
        event = "Training configuration",
        max_epoch = max_epoch,
        mini_batch_size = mini_batch_size,
        max_iteration = max_epoch,
        learning_rate = learning_rate,
    );

    info!(
        event = "Show layer information",
        layers = layers.len(),
        hidden_activation = format!("{:?}", hidden_activation),
        output_activation = format!("{:?}", output_activation)
    );

    let mut trace: Vec<TraceRecord<f64>> = Vec::new();

    let t_0 = Instant::now();
    let mut last_epoch = 1_usize;

    for epoch in 1..(max_epoch + 1) {
        last_epoch = epoch;
        let (grad_list, batch_weight_gradients, loss, trace_record) =
            train(epoch, &train_inputs, &train_answers, &layers);
        trace.push(trace_record);

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

        let early_stop_loss = 0.002;
        if epoch == 1 || epoch % 1000 == 0 || loss < early_stop_loss {
            let mut s = String::from("");
            for layer_no in 0..layers.len() {
                s.push_str(format!(", layer[{layer_no}]={:?}", layers[layer_no]).as_str());
                s.push_str(
                    format!(
                        ", delta[{layer_no}]={}",
                        format!("{:.4}", &batch_weight_gradients[layer_no]).replace("\n", "")
                    )
                    .as_str(),
                );
            }
            info!(epoch = epoch, loss = loss, weight = s);

            // trace biases
            (0..layers.len()).for_each(|i| {
                trace_biases[i].push((epoch, layers[i].bias.flatten().to_vec()));
            })
        }

        if loss < early_stop_loss {
            info!(
                event = "Stop the iterations early because the error is below the target value",
                epoch = epoch,
                loss = loss,
            );
            break;
        }
    }

    // Results
    let elapsed_time = t_0.elapsed();
    info!(
        event = "Results",
        layers = layers.len(),
        mini_batch_size = mini_batch_size,
        last_epoch = last_epoch,
        learning_rate = learning_rate,
        hidden_activation = format!("{:?}", hidden_activation),
        output_activation = format!("{:?}", output_activation),
        elapsed_time = elapsed_time.as_secs(),
        sec_per_epoch = (elapsed_time.as_secs() as f32) / (last_epoch as f32)
    );

    // Trained
    for (i, layer) in layers.iter().enumerate() {
        info!(
            event = "Weights and biases after the training process",
            layer_no = i,
            weight = format!("{:.4?}", &layer.weight).replace("\n", ""),
            bias = format!("{:.4?}", &layer.bias).replace("\n", ""),
        );
    }

    // XOR Predictions
    let mut correct_counts = 0;
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
    let test_batch_size = test_inputs.shape()[1];
    let test_answers = test_inputs
        .map_axis(ndarray::Axis(0), |column| {
            xor_continuous(column[0], column[1])
        })
        .into_shape_with_order((1, test_batch_size))
        .unwrap();
    let mut mse_term = 0.;
    for (i, in_1d_vec_view) in test_inputs.columns().into_iter().enumerate() {
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward(&in_2d_col_vec.view(), &layers);
        let output = &activations.last().unwrap();
        let answer = test_answers[[0, i]];
        let ans11 = ndarray::arr2(&[[answer]]);
        let error = (&ans11 - &output.0).powf(2.).sum() / (in_2d_col_vec.dim().0 as f64);
        mse_term += error;
        if error < 0.05 {
            correct_counts += 1;
        }
        info!(
            event = "XOR predictions",
            inputs = format!("{:.0}", in_1d_vec_view),
            predicted = format!("{:.4}", &output.0[[0, 0]]),
            answer = format!("{:.0}", &output.0[[0, 0]]),
            error = format!("{:.4}", ans11[[0, 0]]),
        );
    }
    let accuracy = correct_counts as f64 / (test_batch_size as f64);
    info!(
        accuracy = accuracy * 100.0,
        rmse = (mse_term / (test_batch_size as f64)).sqrt(),
        layers = layers.len(),
        learning_rate = learning_rate,
        last_epoch = last_epoch,
        mini_batch_size = mini_batch_size,
        hidden_activation = format!("{:?}", hidden_activation),
        output_activation = format!("{:?}", output_activation),
        cosine_similarities = format!(
            "[{:?}]",
            cosine_similarities
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    );
    let train_results = TrainResults::new(
        accuracy,
        (mse_term / (test_batch_size as f64)).sqrt(),
        layers.len(),
        learning_rate,
        mini_batch_size,
        last_epoch,
        &args.hidden_activation,
        &args.output_activation,
    );

    // Append training results to a JSONL file
    let results_json = serde_json::to_string(&train_results).unwrap();
    let results_path = "results/xor_regression_results.jsonl";
    std::fs::create_dir_all("results").ok();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(results_path)
        .unwrap();
    use std::io::Write;
    writeln!(file, "{}", results_json).unwrap();
    info!(
        event = "Appended training results to a JSON file",
        path = results_path
    );

    // Plot traces
    let result;
    if accuracy > 0.95 {
        result = "ok";
    } else {
        result = "ng";
    }
    let date_time = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let image_prefix = format!(
        "{program_name}_{date_time}_{result}_L{:02}_{}",
        layers.len(),
        &args.hidden_activation
    );

    for layer_idx in 0..layers.len() {
        let path = format!("images/{image_prefix}_{layer_idx:02}.png");
        let root_area = BitMapBackend::new(&path, (600, 900)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
        let drawing_areas = root_area.split_evenly((4, 1));
        let mean_area = &drawing_areas[0];
        let var_area = &drawing_areas[1];
        let sim_area = &drawing_areas[2];
        let bias_area = &drawing_areas[3];

        // Number of weights
        let n_weights = trace[0].mean[layer_idx].len();

        let mut chart = ChartBuilder::on(&mean_area)
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
            .build_cartesian_2d(1..last_epoch, -0.1..1.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("epoch")
            .y_desc("weights")
            .draw()
            .ok();

        // Draw chart for each weight
        (0..n_weights).for_each(|w_idx| {
            let color = Palette99::pick(w_idx).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    trace
                        .iter()
                        .map(|v| (v.iteration, v.mean[layer_idx][w_idx])),
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

        let mut chart = ChartBuilder::on(&var_area)
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
            .build_cartesian_2d(1_usize..last_epoch, -0.1..1.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("epoch")
            .y_desc("weights")
            .draw()
            .ok();

        // Draw chart for each weight
        (0..n_weights).for_each(|w_idx| {
            let color = Palette99::pick(w_idx).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    trace
                        .iter()
                        .map(|v| (v.iteration, v.variance[layer_idx][w_idx])),
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

        let mut chart = ChartBuilder::on(&sim_area)
            .caption(
                format!(
                    "Maximum cosine similarity of row vectors: (Layer:{:0}; {:?})",
                    layer_idx, n_weights
                ),
                ("sans-serif", 20),
            )
            .margin(10)
            .margin_right(30)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(1_usize..last_epoch, -0.1..1.0)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("epoch")
            .y_desc("max similarity")
            .draw()
            .ok();

        // Draw chart for each weight
        let color = Palette99::pick(0).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                trace
                    .iter()
                    .map(|v| (v.iteration, v.cosine_similarity_row[layer_idx])),
                color.stroke_width(2),
            ))
            .unwrap()
            .label(format!("max row vec",))
            .legend(move |(x, y)| Rectangle::new([(x, y), (x + 10, y + 1)], color.filled()));

        let color = Palette99::pick(1).mix(0.9);
        chart
            .draw_series(LineSeries::new(
                trace
                    .iter()
                    .map(|v| (v.iteration, v.cosine_similarity_col[layer_idx])),
                color.stroke_width(2),
            ))
            .unwrap()
            .label(format!("max col vec",))
            .legend(move |(x, y)| Rectangle::new([(x, y), (x + 10, y + 1)], color.filled()));

        // Draw legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .ok();

        let num_weights = trace_biases[layer_idx][0].1.len();

        let (y_min, y_max) = trace_biases[layer_idx]
            .iter()
            .flat_map(|(_, b_vec)| b_vec.iter())
            .fold((-0.1, 1.0), |(y_min, y_max), &b| {
                (y_min.min(b), y_max.max(b))
            });
        let mut chart = ChartBuilder::on(&bias_area)
            .caption(
                format!("bias (Layer:{:0}; {:?})", layer_idx, num_weights),
                ("sans-serif", 20),
            )
            .margin(10)
            .margin_right(30)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(1..last_epoch, y_min..y_max)
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|v| format!("{}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("epoch")
            .y_desc("bias")
            .draw()
            .ok();

        // Draw chart for each weight
        (0..num_weights).for_each(|b_idx| {
            let color = Palette99::pick(b_idx).mix(0.9);
            chart
                .draw_series(LineSeries::new(
                    trace_biases[layer_idx].iter().map(|v| (v.0, v.1[b_idx])),
                    color.stroke_width(2),
                ))
                .unwrap()
                .label(format!("b{:?}", b_idx))
                .legend(move |(x, y)| Rectangle::new([(x, y), (x + 10, y + 1)], color.filled()));
        });

        // Draw legend
        chart
            .configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .ok();

        info!("Saved the figure to: {}", path);
    }

    plot_result(&layers, format!("{image_prefix}"));
}
