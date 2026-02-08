use plotters::prelude::*;

fn xor_continuous(x1: f32, x2: f32) -> f32 {
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
        None => "xor_3d_surface_top_view",
    };

    let caption_font = ("sans-serif", 24).into_font();
    let caption_area_height: u32 = caption_font.get_size() as u32;
    let caption_style = plotters::style::TextStyle::from(caption_font.clone()).color(&BLACK);

    let image_path_buf = std::path::PathBuf::from("images").join(format!("{program_name}.png"));
    let image_size: (u32, u32) = (450, caption_area_height + 400);
    let root_area = BitMapBackend::new(&image_path_buf, image_size).into_drawing_area();
    root_area.fill(&WHITE)?;

    let (caption_area, drawing_area) = root_area.split_vertically(caption_area_height as u32);
    caption_area.titled("Continuous XOR Approximation", &caption_style)?;

    let (x_range_min, x_range_max) = (-0.1_f32, 1.0_f32);
    let (y_range_min, y_range_max) = (-0.1_f32, 1.0_f32);
    let (z_range_min, z_range_max) = (-0.1_f32, 1.0_f32);
    let mut chart = ChartBuilder::on(&drawing_area)
        .margin_top(caption_area_height)
        .margin_bottom(30)
        .build_cartesian_3d(
            x_range_min..x_range_max,
            y_range_min..y_range_max,
            z_range_min..z_range_max,
        )?;

    chart.with_projection(|mut p| {
        p.pitch = 0.0 as f64;
        p.yaw = 0.0 as f64;
        p.scale = 1.2 as f64;
        p.into_matrix() // build the projection matrix
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .x_formatter(&|x| format!("x={x:.1}"))
        .y_formatter(&|y| format!("y={y:.1}"))
        .draw()?;

    // step() are not supported in std::ops::RangeInclusive, so add a step value to the end as a workaround.
    let x_step = (x_range_max - x_range_min) / 30_f32;
    let y_step = (y_range_max - y_range_min) / 30_f32;
    chart.draw_series(
        SurfaceSeries::xoy(
            (x_range_min..(x_range_max + x_step)).step(x_step).values(),
            (y_range_min..(y_range_max + y_step)).step(y_step).values(),
            xor_continuous,
        )
        .style_func(&|&v| (VulcanoHSL::get_color(v * 1.0)).into()),
    )?;
    Ok(())
}
