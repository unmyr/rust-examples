use std::env;

use plotters::prelude::*;

// Draw a graph of the inverse sigmoid function (logit)
fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "plotters_ml_logit",
    };

    let image_path_buf = std::path::PathBuf::from("../images").join(format!("{program_name}.png"));

    // Create root drawing area
    let root_area = BitMapBackend::new(&image_path_buf, (640, 800)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // Set x-axis
    let logit = |x: f32| (x / (1_f32 - x)).ln();
    let (x_datapoints_min, x_datapoints_max): (f32, f32) = (0.0001, 0.9999);
    let (x_min, x_max): (f32, f32) = (0_f32, 1_f32);
    let (y_min, y_max): (f32, f32) = (logit(x_datapoints_min), logit(x_datapoints_max));
    let x_datapoints = (x_datapoints_min..x_datapoints_max).step(0.001);

    let color_idx = 1;
    let line_color = Palette99::pick(color_idx).mix(0.9);
    let legend_color = line_color.clone();

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Logit", ("sans-serif", 24))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    let label_name = "logit()";
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .max_light_lines(4)
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        x_datapoints.values().map(|x| (x, logit(x))),
        line_color.stroke_width(2),
    ))
    .unwrap()
    .label(label_name)
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color));
    cc.configure_series_labels()
        .border_style(BLACK)
        .label_font(("Calibri", 20))
        .draw()
        .unwrap();
}
