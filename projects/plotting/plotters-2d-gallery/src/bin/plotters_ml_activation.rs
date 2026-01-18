use std::env;
use std::path::Path;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let cmd_path = match args.get(0) {
        Some(arg0) => Path::new(arg0),
        None => {
            eprintln!("ERROR: Could not retrieve program name.");
            std::process::exit(1)
        }
    };
    let program_name = cmd_path.file_name();
    let program_name = program_name.unwrap().to_str().unwrap();

    let relu = |x: f32| if x > 0.0 { x } else { 0.0 };
    let sigmoid = |x: f32| 1.0 / (1.0 + (-x).exp());
    let tanh = |x: f32| x.tanh();
    let (x_min, x_max): (f32, f32) = (-5., 5.);
    let y_min_vec: Vec<f32> = Vec::from([relu(x_min), sigmoid(x_min), tanh(x_min)]);
    let y_min_work = y_min_vec
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let y_max_vec: Vec<f32> = Vec::from([relu(x_max), sigmoid(x_max), tanh(x_max)]);
    let y_max_work = y_max_vec
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let (y_min, y_max): (f32, f32) = (*y_min_work, *y_max_work);

    // Define legend colors
    let legend_color0: RGBAColor = Palette99::pick(0).mix(0.9);
    let legend_color1: RGBAColor = Palette99::pick(1).mix(0.9);
    let legend_color2: RGBAColor = Palette99::pick(2).mix(0.9);

    let path = format!("../images/{}.svg", program_name);
    let root = plotters::prelude::SVGBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);

    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("ML Activation functions", ("Arial", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .draw()?;

    let x_axis = (x_min..x_max).step(0.1);

    // ReLU function
    let color_idx = 0;
    chart
        .draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, if x > 0.0 { x } else { 0.0 })),
            Palette99::pick(color_idx).mix(0.9).stroke_width(2),
        ))
        .unwrap()
        .label("ReLU")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color0));

    // Draw sigmoid activation function
    let color_idx = 1;
    let line_color = Palette99::pick(color_idx).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, 1.0 / (1.0 + (-x).exp()))),
            line_color.stroke_width(2),
        ))
        .unwrap()
        .label("Sigmoid")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color1));

    // Draw hyperbolic tangent (tanh) activation function
    let color_idx = 2;
    let line_color = Palette99::pick(color_idx).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.tanh())),
            line_color.stroke_width(2),
        ))
        .unwrap()
        .label("Tanh")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color2));

    // Draw legend
    chart
        .configure_series_labels()
        .border_style(BLACK)
        .label_font(("Calibri", 20))
        .draw()?;

    Ok(())
}
