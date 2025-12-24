use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2};
use plotters::prelude::*;
use rand::Rng;

fn xor_continuous(x1: f64, x2: f64) -> f64 {
    x1 + x2 - 2.0 * x1 * x2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate dataset
    let mut rng = rand::rng();
    let n_samples = 1000;

    let mut inputs = Array2::<f64>::zeros((n_samples, 3));
    let mut targets = Array1::<f64>::zeros(n_samples);

    for i in 0..n_samples {
        let x1 = rng.random_range(0.0..=1.0);
        let x2 = rng.random_range(0.0..=1.0);
        let y = xor_continuous(x1, x2);

        // The features are:
        // y = w_0 + w_1*x_1 + w_2*x_2 + w_3(x_1*x_2)
        inputs[[i, 0]] = x1;
        inputs[[i, 1]] = x2;
        inputs[[i, 2]] = x1 * x2; // Interaction term
        targets[i] = y;
    }

    // Create Dataset
    let dataset = Dataset::<_, _, _>::new(inputs, targets);

    // Training with Linear Regression
    let model = LinearRegression::default().fit(&dataset).unwrap();

    // Test predictions
    let test_inputs = ndarray::array![
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 1.0]
    ];
    let predictions: Array1<f64> = model.predict(&test_inputs);

    println!("== XOR Predictions ==");
    for (input, pred) in test_inputs.outer_iter().zip(predictions.iter()) {
        println!("Input: {:?} => Predicted: {:.3}", input.to_vec(), pred);
    }

    let root = BitMapBackend::gif("images/xor_reg1.gif", (600, 400), 100)?.into_drawing_area();
    for pitch in 0..157 {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Continuous XOR Approximation", ("sans-serif", 20))
            .build_cartesian_3d(-0.1..1.0, -0.1..1.0, -0.1..1.0)?;
        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart.draw_series(
            SurfaceSeries::xoz(
                (-2..=20).map(|i| i as f64 * 0.05),
                (-2..=20).map(|i| i as f64 * 0.05),
                xor_continuous,
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v * 1.0)).into()),
        )?;

        root.present()?;
    }
    Ok(())
}
