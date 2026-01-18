use std::env;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "plotters_multiple_plots_tanh",
    };

    let image_path = format!("../images/{program_name}.png");

    // Create root drawing area
    let root_area = BitMapBackend::new(&image_path, (640, 800)).into_drawing_area();
    let (upper, lower) =
        root_area.split_vertically(((root_area.dim_in_pixel().1 as f32) / 2_f32) as u32);
    root_area.fill(&WHITE)?;

    // Set x-axis
    let (x_min, x_max): (f32, f32) = (-5., 5.);
    let (y_min, y_max): (f32, f32) = (-1., 1.);
    let x_axis = (x_min..x_max).step(0.1);

    // Plotting a graph of the `tanh` function
    let color_idx = 1;
    let line_color = Palette99::pick(color_idx).mix(0.9);
    let legend_color = line_color.clone();
    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("The hyperbolic tangent (tanh) function", ("sans-serif", 20))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    let label_name = "tanh";
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .max_light_lines(4)
        .draw()?;
    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| (x, x.tanh())),
        line_color.stroke_width(2),
    ))
    .unwrap()
    .label(label_name)
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color));
    // Draw legend
    cc.configure_series_labels()
        .border_style(BLACK)
        .label_font(("Calibri", 20))
        .draw()?;

    // The derivative of the tanh function
    let color_idx = 2;
    let line_color = Palette99::pick(color_idx).mix(0.9);
    let legend_color = line_color.clone();
    let mut cc = ChartBuilder::on(&lower)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(
            "Derivative of the hyperbolic tangent (tanh) function'",
            ("sans-serif", 20),
        )
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    let label_name = "tanh'";
    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .max_light_lines(4)
        .draw()?;
    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| {
            let s = 1.0 / (1.0 + (-x).exp());
            (x, s * (1. - s))
        }),
        line_color.stroke_width(2),
    ))
    .unwrap()
    .label(label_name)
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color.clone()));
    // Draw legend
    cc.configure_series_labels()
        .border_style(BLACK)
        .label_font(("Calibri", 20))
        .draw()?;

    Ok(())
}
