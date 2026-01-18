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
        None => "plotters_multiple_plots_h3",
    };

    // Define output image path
    let image_path = format!("../images/{program_name}.svg");

    // Create root drawing area
    let root_area = SVGBackend::new(&image_path, (800, 600)).into_drawing_area();
    let drawing_areas = root_area.split_evenly((1, 3));
    root_area.fill(&WHITE)?;
    println!("Drawing areas len={}", drawing_areas.len());

    // Set x-axis and y-axis ranges
    let (x_min, x_max): (f32, f32) = (-2., 2.);
    let (y_min, y_max): (f32, f32) = (-8., 8.);
    let x_axis = (x_min..x_max).step(0.1);

    // Plotting a graph of x^n function
    for (idx, area) in drawing_areas.iter().enumerate() {
        let color_idx = idx;
        let line_color = Palette99::pick(color_idx).mix(0.9);
        let legend_color = line_color.clone();
        let label_name = format!("x^{}", idx + 1);

        // Setup chart context
        let mut cc = ChartBuilder::on(&area)
            .margin(5)
            .set_all_label_area_size(50)
            .caption(&label_name, ("sans-serif", 24))
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        // Draw mesh and axes
        cc.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()?;

        // Draw x^n curve
        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.powi((idx + 1) as i32))),
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
    }
    println!("Result has been saved to {}", image_path);
    Ok(())
}
