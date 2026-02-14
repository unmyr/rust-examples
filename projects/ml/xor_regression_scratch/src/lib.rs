use std::io::Write;
use num_traits::{Float, FromPrimitive};

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

// Activation functions supported
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Activation {
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

// Layer structure
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LayerConfig {
    pub input_dim: usize,
    pub output_dim: usize,
    pub activation: Activation,
}

impl LayerConfig {
    pub fn new(input_dim: usize, output_dim: usize, activation: Activation) -> LayerConfig {
        LayerConfig {
            input_dim,
            output_dim,
            activation,
        }
    }
}

// Parameters of individual layers in neural network training
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LayerParams<F: Float> {
    pub weight: ndarray::Array2<F>,
    pub bias: ndarray::Array2<F>,
}

// Implement constructor for LayerParams
impl<F: Float> LayerParams<F> {
    pub fn new(weight: ndarray::Array2<F>, bias: ndarray::Array2<F>) -> LayerParams<F> {
        LayerParams { weight, bias }
    }
}

// Model learning state (to be saved/restored)
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModelState<F: Float> {
    layers: Vec<LayerParams<F>>,
}

// Implement constructor for ModelState
impl<F: Float + serde::Serialize + for<'a> serde::Deserialize<'a>> ModelState<F> {
    pub fn from_params(params: Vec<LayerParams<F>>) -> ModelState<F> {
        ModelState { layers: params }
    }

    pub fn from_nn(nn: &NeuralNetwork<F>) -> ModelState<F> {
        let layers = nn.layers.iter().map(|l| l.params.clone()).collect();
        ModelState { layers }
    }

    pub fn save(&self, path: &std::path::Path) {
        let mut file = std::fs::File::create(path).unwrap();
        let serialized_str = serde_json::to_string(self).unwrap();
        file.write_all(serialized_str.as_bytes()).unwrap();
    }

    pub fn load(path: &std::path::Path) -> ModelState<F> {
        let file_content = std::fs::read_to_string(path).unwrap();
        let model_state: ModelState<F> = serde_json::from_str(&file_content).unwrap();
        model_state
    }
}

// Layer structure combining configuration and parameters
pub struct Layer<F: Float> {
    pub config: LayerConfig,
    pub params: LayerParams<F>,
}

impl<F: Float> Layer<F> {
    pub fn new(config: LayerConfig, params: LayerParams<F>) -> Layer<F> {
        Layer { config, params }
    }
}

impl<F: std::fmt::Debug + Float> std::fmt::Debug for Layer<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{weight: {}, bias: {}, activation: {:?}}}",
            format!("{:.4?}", self.params.weight.view()).replace("\n", ""),
            format!("{:.4?}", self.params.bias.view()).replace("\n", ""),
            self.config.activation
        )
    }
}

// Configuration of the neural network model
pub struct NeuralNetwork<F: Float> {
    pub layers: Vec<Layer<F>>,
    learning_rate: F,
}

impl<F: Float> NeuralNetwork<F> {
    pub fn from_config_and_state_and_lr(
        config: Vec<LayerConfig>,
        params: ModelState<F>,
        learning_rate: F,
    ) -> NeuralNetwork<F> {
        let layers: Vec<Layer<F>> = config
            .into_iter()
            .zip(params.layers.into_iter())
            .map(|(c, p)| Layer::new(c, p))
            .collect();
        NeuralNetwork {
            layers,
            learning_rate,
        }
    }
}

// Load and save hyperparameters and model parameters
impl<F: num_traits::Float> NeuralNetwork<F> {
    pub fn save_config(&self, path: &std::path::Path) {
        let mut file = std::fs::File::create(path).unwrap();
        let config: Vec<LayerConfig> = self.layers.iter().map(|l| l.config.clone()).collect();
        let serialized_str = serde_json::to_string(&config).unwrap();
        file.write_all(serialized_str.as_bytes()).unwrap();
    }

    pub fn load_config(path: &str) -> Vec<LayerConfig> {
        let file_content = std::fs::read_to_string(path).unwrap();
        let layer_configs: Vec<LayerConfig> = serde_json::from_str(&file_content).unwrap();
        layer_configs
    }
}

impl<F: num_traits::Float + 'static> NeuralNetwork<F> {
    // Forward propagation
    pub fn forward(
        &self,
        input: &ndarray::ArrayView2<F>,
    ) -> Vec<(ndarray::Array2<F>, Activation, ndarray::Array2<F>)> {
        let current_input = input.clone().into_owned();
        let mut activations = vec![(
            current_input.clone(),
            Activation::Identity,
            current_input.clone(),
        )];

        for layer in self.layers.iter() {
            let w_out = &layer
                .params
                .weight
                .dot(&activations.last().unwrap().0.view())
                + &layer.params.bias;
            let pre_activation = w_out.clone();
            let w_out_s = match &layer.config.activation {
                Activation::Identity => w_out.mapv(identity),
                Activation::ReLU => w_out.mapv(relu),
                Activation::Sigmoid => w_out.mapv(sigmoid),
                Activation::Tanh => w_out.mapv(tanh),
            };
            let current_input = w_out_s;
            activations.push((
                current_input,
                layer.config.activation.clone(),
                pre_activation,
            ));
        }

        activations
    }
}

impl<F: num_traits::Float + std::fmt::Debug + FromPrimitive + 'static> NeuralNetwork<F> {
    // Training function
    pub fn train(
        &mut self,
        epoch: usize,
        train_inputs: &ndarray::Array2<F>,
        train_answers_ref: &ndarray::Array2<F>,
    ) -> (
        Vec<ndarray::Array2<F>>,
        Vec<ndarray::Array2<F>>,
        F,
        TraceRecord<F>,
    ) {
        let mini_batch_size = train_inputs.shape()[1];
        // Squared errors in the output layer
        let mut loss_terms = ndarray::Array2::<F>::zeros((
            self.layers.last().unwrap().params.weight.shape()[0],
            mini_batch_size,
        ));

        let mut grad_list: Vec<ndarray::Array2<F>> = Vec::new();
        let mut batch_weight_gradients: Vec<ndarray::Array2<F>> = Vec::new();
        let mut trace_outputs: Vec<ndarray::Array2<F>> = Vec::new();
        let mut pre_activation_outputs: Vec<ndarray::Array2<F>> = Vec::new();

        // Accumulate the gradient for each sample
        (0..self.layers.len()).for_each(|i| {
            grad_list.push(ndarray::Array2::zeros(self.layers[i].params.weight.dim()));
            let output_size = self.layers[i].params.weight.dim().0;
            batch_weight_gradients.push(ndarray::Array2::<F>::zeros((output_size, 1)));
            trace_outputs.push(ndarray::Array2::zeros((output_size, mini_batch_size)));
            pre_activation_outputs.push(ndarray::Array2::zeros((output_size, mini_batch_size)));
        });

        for (i, in_1d_vec_view) in train_inputs.columns().into_iter().enumerate() {
            let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
            let activations = self.forward(&in_2d_col_vec.view());
            pre_activation_outputs
                .iter_mut()
                .enumerate()
                .for_each(|(layer_no, pre_act_out)| {
                    let a_idx = layer_no + 1;
                    let pre_act = &activations[a_idx].2;
                    pre_act_out
                        .column_mut(i)
                        .assign(&pre_act.clone().remove_axis(ndarray::Axis(1)));
                });

            let mut cur_gradients;
            cur_gradients = &activations.last().unwrap().0 - &train_answers_ref.column(i);
            loss_terms
                .column_mut(i)
                .assign(&cur_gradients.column(0).powf(F::one() + F::one()));

            for layer_no in (0..self.layers.len()).rev().into_iter() {
                let a_idx = layer_no + 1;
                let activation = &activations[a_idx].1;
                let l_input = &activations[a_idx - 1].0;
                let l_output = &activations[a_idx].0;

                // N-by-1 matrix representing the gradient
                let delta = match activation {
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
                cur_gradients = self.layers[layer_no].params.weight.t().dot(&delta);
            }
        }

        // Update weight and bias
        (0..self.layers.len()).for_each(|i| {
            self.layers[i].params.weight.scaled_add(
                -F::one() * self.learning_rate / F::from(mini_batch_size).unwrap(),
                &grad_list[i],
            );
            self.layers[i].params.bias.scaled_add(
                -F::one() * self.learning_rate / F::from(mini_batch_size).unwrap(),
                &batch_weight_gradients[i],
            );
        });

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
        for layer_no in (0..self.layers.len()).into_iter() {
            if &self.layers[layer_no].params.weight.dim().0 == &1_usize {
                trace_row_sim.push(F::zero());
                trace_col_sim.push(F::zero());
                continue;
            }

            let sim_row_vec = cosine_similarity_vec(&self.layers[layer_no].params.weight.view());
            let max_row_idx = sim_row_vec
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            trace_row_sim.push(sim_row_vec[max_row_idx]);

            let sim_col_vec = cosine_similarity_vec(&self.layers[layer_no].params.weight.t());
            let max_col_idx = sim_col_vec
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            trace_col_sim.push(sim_col_vec[max_col_idx]);
        }

        let trace = TraceRecord::new(
            epoch,
            trace_mean,
            trace_var,
            trace_row_sim,
            trace_col_sim,
            pre_activation_outputs,
        );
        let loss = loss_terms.sum() / F::from(mini_batch_size).unwrap();
        (grad_list, batch_weight_gradients, loss, trace)
    }
}

// Record of trace information during training
#[derive(Debug)]
pub struct TraceRecord<F: Float> {
    pub iteration: usize,
    pub mean: Vec<ndarray::Array1<F>>,
    pub variance: Vec<ndarray::Array1<F>>,
    pub cosine_similarity_row: Vec<F>,
    pub cosine_similarity_col: Vec<F>,
    pub pre_activation_outputs: Vec<ndarray::Array2<F>>,
}

// Implement constructor for TraceRecord
impl<F: Float> TraceRecord<F> {
    pub fn new(
        iteration: usize,
        mean: Vec<ndarray::Array1<F>>,
        variance: Vec<ndarray::Array1<F>>,
        cosine_similarity_row: Vec<F>,
        cosine_similarity_col: Vec<F>,
        pre_activation_outputs: Vec<ndarray::Array2<F>>,
    ) -> TraceRecord<F> {
        TraceRecord {
            iteration: iteration,
            mean: mean,
            variance: variance,
            cosine_similarity_row: cosine_similarity_row,
            cosine_similarity_col: cosine_similarity_col,
            pre_activation_outputs: pre_activation_outputs,
        }
    }
}
