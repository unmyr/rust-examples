use plotters::prelude::*;
use rand::distr::Distribution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => {
            let cmd_path = std::path::Path::new(arg0);
            let program_name = cmd_path.file_name();
            let program_name = program_name.unwrap().to_str().unwrap();
            program_name
        }
        None => "plotters_normal_dist_prob",
    };

    let mut rng = rand::rng();
    let (mean, std_dev): (f64, f64) = (0., 0.6);
    let norm_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    let num_samples = 100000;
    let x_iter = norm_dist.sample_iter(&mut rng);
    let sample_values: Vec<f64> = x_iter.take(num_samples).collect();
    let (x_min, x_max): (f64, f64) = (
        *sample_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        *sample_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );
    println!("x_min: {:.2}, x_max: {:.2}", x_min, x_max);
    let (y_min, y_max): (f32, f32) = (0., 100. / 10.);

    let path = format!("../images/{}.png", program_name);

    let root = plotters::prelude::BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 20, 10);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!(
                "Normal distribution (mean={:.2}, std_dev={:.2}, {:?} points)",
                mean, std_dev, num_samples
            ),
            ("Arial", 20).into_font(),
        )
        .x_label_area_size(20)
        .y_label_area_size(60)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?
        .set_secondary_coord(
            (x_min..x_max).step(0.1).use_round().into_segmented(),
            y_min..y_max,
        );
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Probability(%)")
        .draw()?;

    let actual = Histogram::vertical(chart.borrow_secondary())
        .style(GREEN.filled())
        .margin(3)
        .data(
            sample_values
                .iter()
                .map(|x| (*x, 100. * 1. / (num_samples as f32))),
        );

    chart
        .draw_secondary_series(actual)?
        .label("Observed")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.filled()));

    chart.configure_series_labels().draw()?;
    Ok(())
}
