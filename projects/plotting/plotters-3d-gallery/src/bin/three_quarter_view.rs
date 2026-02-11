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
        None => "rotate_3d",
    };

    let points = ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

    let image_path = format!("images/{}.svg", program_name);
    let drawing_area = SVGBackend::new(&image_path, (1024, 760)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let caption_font = ("sans-serif", 20).into_font();
    let caption_area_height: u32 = caption_font.get_size() as u32;
    let caption_style = plotters::style::TextStyle::from(caption_font.clone()).color(&BLACK);

    let area = drawing_area
        .split_evenly((2, 1))
        .iter()
        .map(|area| area.split_evenly((1, 2)))
        .collect::<Vec<_>>();

    let x_axis = (0.0_f32..1.1_f32).step(0.5);
    let y_axis = (0.0_f32..1.1_f32).step(0.5);
    let z_axis = (0.0_f32..1.1_f32).step(0.5);

    // Three-quarter view
    let common_margin = 10 as u32;
    let dot_and_label = |p2: (f32, f32), p3: (f32, f32, f32)| {
        let circle = if p3.0 == 1.0 {
            Circle::new((0, 0), 3, ShapeStyle::from(&GREEN).filled())
        } else if p3.1 == 1.0 {
            Circle::new((0, 0), 3, ShapeStyle::from(&BLUE).filled())
        } else {
            Circle::new((0, 0), 3, ShapeStyle::from(&RED).filled())
        };

        return EmptyElement::at((p2.0, p2.1))
            + circle
            + Text::new(
                format!("{:.0?}", p3),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    // Y-Z Plane: Right side view
    let (caption_area, drawing_area) = &area[0][0].split_vertically(caption_area_height as u32);
    caption_area.titled("Right side view", &caption_style)?;

    let mut cc = ChartBuilder::on(&drawing_area)
        .margin(common_margin)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(y_axis.clone(), z_axis.clone())?;

    cc.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .x_desc("Y")
        .y_desc("Z")
        .draw()?;

    cc.draw_series(points.clone().map_axis(ndarray::Axis(1), |row| {
        dot_and_label((row[1], row[2]), (row[0], row[1], row[2]))
    }))?;

    // X-Y Plane: Top view
    let (caption_area, drawing_area) = &area[0][1].split_vertically(caption_area_height as u32);
    caption_area.titled("Top view(pitch=0.0,yaw=0.0)", &caption_style)?;

    let mut cc = ChartBuilder::on(&drawing_area)
        .margin(common_margin)
        .margin_bottom(30)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    cc.with_projection(|mut pb| {
        pb.pitch = 0.0 as f64;
        pb.yaw = 0.0 as f64;
        pb.scale = 1.2 as f64;
        pb.into_matrix() // build the projection matrix
    });

    cc.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .x_formatter(&|x| format!("x={x:.1}"))
        .y_formatter(&|y| format!("y={y:.1}"))
        .draw()?;

    cc.draw_series(points.map_axis(ndarray::Axis(1), |row| {
        if row[0] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, GREEN.filled()).into_dyn()
        } else if row[1] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, BLUE.filled()).into_dyn()
        } else {
            Cross::new((row[0], row[1], row[2]), 4, RED.filled()).into_dyn()
        }
    }))?;

    // X-Z Plane: Front view
    let (caption_area, drawing_area) = &area[1][0].split_vertically(caption_area_height as u32);
    caption_area.titled("Front view(pitch= -π/2,yaw=0.0)", &caption_style)?;

    let mut cc = ChartBuilder::on(&drawing_area)
        .margin(common_margin)
        .margin_bottom(30)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    cc.with_projection(|mut pb| {
        pb.pitch = -std::f64::consts::FRAC_PI_2;
        pb.yaw = 0.0 as f64;
        pb.scale = 1.2 as f64;
        pb.into_matrix() // build the projection matrix
    });

    cc.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .x_formatter(&|x| format!("x={x:.1}"))
        .z_formatter(&|z| format!("z={z:.1}"))
        .draw()?;

    cc.draw_series(points.map_axis(ndarray::Axis(1), |row| {
        if row[0] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, GREEN.filled()).into_dyn()
        } else if row[1] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, BLUE.filled()).into_dyn()
        } else {
            Cross::new((row[0], row[1], row[2]), 4, RED.filled()).into_dyn()
        }
    }))?;

    // 3D View
    let mut cc = ChartBuilder::on(&area[1][1])
        .margin(common_margin)
        .margin_bottom(30)
        .caption("3D view", ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    cc.with_projection(|mut pb| {
        // pb.yaw = std::f64::consts::FRAC_PI_8;
        // pb.pitch = -std::f64::consts::FRAC_PI_2;
        // Top view
        pb.pitch = 0.;
        pb.yaw = 0.;
        // Front view
        pb.pitch = std::f64::consts::FRAC_PI_8 * 0.5;
        pb.yaw = std::f64::consts::FRAC_PI_3;
        pb.scale = 0.8;
        pb.into_matrix()
    });

    cc.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .x_formatter(&|x| format!("x={x:.1}"))
        .y_formatter(&|y| format!("y={y:.1}"))
        .z_formatter(&|z| format!("z={z:.1}"))
        .draw()?;

    cc.draw_series(points.map_axis(ndarray::Axis(1), |row| {
        if row[0] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, GREEN.filled()).into_dyn()
        } else if row[1] == 1.0 {
            Circle::new((row[0], row[1], row[2]), 4, BLUE.filled()).into_dyn()
        } else {
            Cross::new((row[0], row[1], row[2]), 4, RED.filled()).into_dyn()
        }
    }))?;

    Ok(())
}
