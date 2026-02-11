use plotters::prelude::*;

fn xor_continuous(x1: f64, x2: f64) -> f64 {
    x1 + x2 - 2.0 * x1 * x2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "xor_3d_surface",
    };
    let image_path_buf = std::path::PathBuf::from("images").join(format!("{program_name}.gif"));
    let root = BitMapBackend::gif(&image_path_buf, (600, 400), 100)?.into_drawing_area();
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
            // axis label formatters is not working yet on animation chart
            .x_formatter(&|x| format!("{:.1}", x))
            .y_formatter(&|y| format!("{:.1}", y))
            .z_formatter(&|z| format!("{:.1}", z))
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
    println!("Result has been saved to {}", image_path_buf.display());
    Ok(())
}
