use std::time::Instant;

use rand_distr::Distribution;
#[derive(Debug)]
struct LayerConfig<F>
where
    F: num_traits::Float,
{
    weight: ndarray::Array2<F>,
    bias: ndarray::Array2<F>,
}

impl<F> LayerConfig<F>
where
    F: num_traits::Float,
    rand_distr::StandardNormal: rand_distr::Distribution<F>,
{
    pub fn new(
        input_size: usize,
        output_size: usize,
        rng: &mut rand::rngs::ThreadRng,
        bias: F,
    ) -> LayerConfig<F> {
        let (mean, std_dev): (F, F) = (F::zero(), F::from(0.5).unwrap());
        let normal_dist = rand_distr::Normal::<F>::new(mean, std_dev).unwrap();
        LayerConfig {
            weight: ndarray::Array2::from_shape_fn((output_size, input_size), |_| {
                normal_dist.sample(rng)
            }),
            bias: ndarray::Array2::from_shape_fn((output_size, 1), |_| bias),
        }
    }
}

fn forward<F: num_traits::Float + 'static>(
    input: &ndarray::ArrayView2<F>,
    layers: &Vec<LayerConfig<F>>,
) -> Vec<ndarray::Array2<F>> {
    let current_input = input.clone().into_owned();
    let mut activations = vec![current_input];

    for layer in layers.iter() {
        let last_output = activations.last().unwrap();
        let w_out = &layer.weight.dot(last_output) + &layer.bias;
        let w_out_s = w_out.mapv(|x| F::one() / (F::one() + (-x).exp()));
        let current_input = w_out_s;
        activations.push(current_input);
    }

    activations
}

fn train<F: num_traits::Float + std::fmt::Display + 'static>(
    epoch: usize,
    train_inputs: &ndarray::Array2<F>,
    train_answers_ref: &ndarray::Array2<F>,
    layers: &mut Vec<LayerConfig<F>>,
    learning_rate: &F,
) -> F {
    let mini_batch_size = train_inputs.shape()[1];
    let mut loss_terms =
        ndarray::Array2::<F>::zeros((layers.last().unwrap().weight.shape()[0], mini_batch_size));

    let mut grad_list: Vec<ndarray::Array2<F>> = Vec::new();
    let mut batch_weight_gradients: Vec<ndarray::Array2<F>> = Vec::new();

    // Accumulate the gradient for each sample
    (0..layers.len()).for_each(|i| {
        grad_list.push(ndarray::Array2::zeros(layers[i].weight.dim()));
        batch_weight_gradients.push(ndarray::Array2::<F>::zeros((
            layers[i].weight.shape()[0],
            1,
        )));
    });

    for (i, in_1d_vec_view) in train_inputs.columns().into_iter().enumerate() {
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward::<F>(&in_2d_col_vec.view(), layers);

        let mut cur_gradients;
        cur_gradients = *&activations.last().unwrap() - &train_answers_ref.column(i);
        loss_terms
            .column_mut(i)
            .assign(&cur_gradients.column(0).powf(F::one() + F::one()));

        for layer_no in (0..layers.len()).rev().into_iter() {
            let a_idx = layer_no + 1;
            let l_input = &activations[a_idx - 1];
            let l_output = &activations[a_idx];

            let delta = &cur_gradients * l_output.mapv(|s| s * (F::one() - s));

            // Calculate gradients in a loop (using cross products)
            grad_list[layer_no].scaled_add(F::one(), &delta.dot(&l_input.t()));
            batch_weight_gradients[layer_no]
                .column_mut(0)
                .scaled_add(F::one(), &delta.column(0));

            // Next error inputs
            cur_gradients = layers[layer_no].weight.t().dot(&delta);
        }
    }

    let mini_batch_size = F::from(mini_batch_size).unwrap();
    let loss = loss_terms.sum() / mini_batch_size;

    // Update weight and bias
    (0..layers.len()).for_each(|i| {
        layers[i].weight.scaled_add(
            -F::one(),
            &(&grad_list[i].mapv(|v| v * *learning_rate / mini_batch_size)),
        );
        layers[i].bias.scaled_add(
            -F::one(),
            &batch_weight_gradients[i].mapv(|v| v * *learning_rate / mini_batch_size),
        );
    });

    if epoch % 4000 == 0 {
        print!(
            "epoch={:06}, mini_batch_size={}, loss={:.4}",
            epoch, mini_batch_size, loss
        );
        for layer_no in 0..layers.len() {
            print!(
                ", delta[{layer_no}]^T={:.4}",
                &batch_weight_gradients[layer_no].t()
            );
        }
        println!("");
    }

    loss
}

fn main() {
    let mut rng = rand::rng();
    let mut layers: Vec<LayerConfig<f64>> = Vec::new();

    let input_size: usize = 2;
    let output_size: usize = 2;
    let layer = LayerConfig::<f64>::new(input_size, output_size, &mut rng, 0.5);
    println!("Layer: {:?}", layer);
    layers.push(layer);

    let input_size: usize = output_size;
    let output_size: usize = 1;
    let layer = LayerConfig::<f64>::new(input_size, output_size, &mut rng, 0.);
    println!("Layer: {:?}", layer);
    layers.push(layer);

    let learning_rate = 0.5;
    let max_epoch: usize = 20000;
    let train_inputs: ndarray::Array2<f64> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
    let mini_batch_size = train_inputs.dim().1;
    let train_answers = train_inputs
        .map_axis(ndarray::Axis(0), |column| {
            let (x1, x2) = (column[0], column[1]);
            x1 + x2 - 2. * x1 * x2
        })
        .into_shape_with_order((1, mini_batch_size))
        .unwrap();

    let t_0 = Instant::now();
    let last_epoch = 0_usize;
    for epoch in 1..(max_epoch + 1) {
        let loss = train(
            epoch,
            &train_inputs,
            &train_answers,
            &mut layers,
            &learning_rate,
        );

        if loss < 0.002 {
            println!(
                "INFO: Early stopping at epoch={} due to small loss={:.4}",
                epoch, loss
            );
            break;
        }
    }

    println!("=== Results");
    let elapsed_time = t_0.elapsed();
    println!(
        "last_epoch={last_epoch}, mini_batch_size={mini_batch_size}, learning_rate={}, elapsed time={:.2}[s] {:.6}[s/epoch]",
        learning_rate,
        elapsed_time.as_secs() as f32,
        (elapsed_time.as_secs() as f32) / (last_epoch as f32)
    );

    println!("\n== XOR Predictions ==");
    let mut correct_counts = 0;
    let test_inputs = ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();
    let test_batch_size = test_inputs.shape()[1];
    let test_answers = test_inputs
        .map_axis(ndarray::Axis(0), |column| {
            let (x1, x2) = (column[0], column[1]);
            x1 + x2 - 2. * x1 * x2
        })
        .into_shape_with_order((1, test_batch_size))
        .unwrap();
    for (i, in_1d_vec_view) in test_inputs.columns().into_iter().enumerate() {
        let (x1, x2) = (in_1d_vec_view[0], in_1d_vec_view[1]);
        let in_2d_col_vec = in_1d_vec_view.insert_axis(ndarray::Axis(1));
        let activations = forward(&in_2d_col_vec.view(), &layers);
        let answer = test_answers[[0, i]];
        let ans11 = ndarray::arr2(&[[answer]]);
        let loss = (&ans11 - &activations[2]).powf(2.).sum() / 2.;
        if loss < 0.05 {
            correct_counts += 1;
        }
        println!(
            "Input: [{:?}] => Predicted: {:.2}, answer: {:.0}, loss: {:.2}",
            [x1, x2],
            &activations[2][[0, 0]],
            ans11[[0, 0]],
            loss
        );
    }
    println!(
        "Accuracy: {:.2}%",
        (correct_counts as f64 / (test_batch_size as f64)) * 100.0
    );
}
