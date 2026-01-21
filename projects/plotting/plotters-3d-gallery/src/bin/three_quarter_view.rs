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

    let points = ndarray::arr2(&[[1., 0., 0.]]);

    let image_path = format!("images/{}.svg", program_name);
    let drawing_area = SVGBackend::new(&image_path, (1024, 760)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let area = drawing_area
        .split_evenly((2, 1))
        .iter()
        .map(|area| area.split_evenly((1, 2)))
        .collect::<Vec<_>>();

    let x_axis = (0.0_f32..1.0_f32).step(0.1);
    let y_axis = (0.0_f32..1.0_f32).step(0.1);
    let z_axis = (0.0_f32..1.0_f32).step(0.1);

    // Three-quarter view
    let common_margin = 10 as u32;
    let dot_and_label = |p2: (f32, f32), p3: (f32, f32, f32)| {
        return EmptyElement::at((p2.0, p2.1))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("{:.2?}", p3),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    // Y-Z Plane: Right side view
    let mut cc = ChartBuilder::on(&area[0][0])
        .margin(common_margin)
        .caption("Right side view", ("sans", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 100)
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
    let mut cc = ChartBuilder::on(&area[0][1])
        .margin(common_margin)
        .caption("Top view", ("sans", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 100)
        .build_cartesian_2d(x_axis.clone(), y_axis.clone())?;

    cc.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .x_desc("X")
        .y_desc("Y")
        .draw()?;

    cc.draw_series(points.clone().map_axis(ndarray::Axis(1), |row| {
        dot_and_label((row[0], row[1]), (row[0], row[1], row[2]))
    }))?;

    // X-Z Plane: Front view
    let mut cc = ChartBuilder::on(&area[1][0])
        .margin(common_margin)
        .caption("Front view", ("sans", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 100)
        .build_cartesian_2d(x_axis.clone(), z_axis.clone())?;

    cc.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.1}", x))
        .x_desc("X")
        .y_desc("Z")
        .draw()?;

    cc.draw_series(points.clone().map_axis(ndarray::Axis(1), |row| {
        dot_and_label((row[0], row[2]), (row[0], row[1], row[2]))
    }))?;

    // 3D View
    let mut cc = ChartBuilder::on(&area[1][1])
        .margin(common_margin)
        .caption("3D view", ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    cc.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 1.0;
        pb.into_matrix()
    });

    cc.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    cc.draw_series(points.map_axis(ndarray::Axis(1), |row| {
        Circle::new((row[0], row[1], row[2]), 2, GREEN.filled())
    }))?;

    Ok(())
}
