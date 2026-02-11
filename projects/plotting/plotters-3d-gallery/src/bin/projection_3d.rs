use std::env;

use fraction::Fraction;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "projection_3d",
    };

    let points = ndarray::arr2(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

    let caption_font = ("sans-serif", 20).into_font();
    let caption_area_height: u32 = caption_font.get_size() as u32;
    let caption_style = plotters::style::TextStyle::from(caption_font.clone()).color(&BLACK);

    let x_axis = (0.0_f32..1.0_f32).step(0.5);
    let y_axis = (0.0_f32..1.0_f32).step(0.5);
    let z_axis = (0.0_f32..1.0_f32).step(0.5);

    let common_margin = 10 as u32;
    let margin_top = 20 as u32;
    let margin_bottom = 30 as u32;
    let common_scale = 0.8 as f64;
    let image_size = (1024 * 2, 768 * 4);

    let image_path = format!("images/{}_pitch_yaw.svg", program_name);
    let drawing_area = SVGBackend::new(&image_path, image_size).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let i_divisions: usize = 8;
    let j_divisions: usize = 8;
    let areas = drawing_area
        .split_evenly((i_divisions, 1))
        .iter()
        .map(|area| area.split_evenly((1, j_divisions)))
        .collect::<Vec<_>>();

    let pitch_radians_factor = match i_divisions {
        2 => std::f64::consts::PI,
        4 => std::f64::consts::FRAC_PI_2,
        8 => std::f64::consts::FRAC_PI_4,
        16 => std::f64::consts::FRAC_PI_8,
        _ => std::f64::consts::FRAC_PI_8,
    };
    let yaw_radians_factor = match j_divisions {
        2 => std::f64::consts::PI,
        4 => std::f64::consts::FRAC_PI_2,
        8 => std::f64::consts::FRAC_PI_4,
        16 => std::f64::consts::FRAC_PI_8,
        _ => std::f64::consts::FRAC_PI_8,
    };
    for i in 0..areas.len() {
        for j in 0..areas[0].len() {
            let (caption_area, drawing_area) =
                &areas[i][j].split_vertically(caption_area_height as u32);

            // Caption area shows the fraction of π for pitch and yaw
            let fraction_of_i_pi = Fraction::from(i) / Fraction::from(i_divisions / 2);
            let fraction_of_j_pi = Fraction::from(j) / Fraction::from(j_divisions / 2);
            let (pi_str_i, pi_str_j) =
                (if i == 0 { "" } else { "π" }, if j == 0 { "" } else { "π" });
            caption_area.titled(
                &format!("p:{fraction_of_i_pi}{pi_str_i}, y:{fraction_of_j_pi}{pi_str_j}"),
                &caption_style,
            )?;

            // Drawing area
            let mut cc = ChartBuilder::on(&drawing_area)
                .margin(common_margin)
                .margin_top(margin_top)
                .margin_bottom(margin_bottom)
                .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

            cc.with_projection(|mut pb| {
                pb.pitch = (i as f64) * pitch_radians_factor;
                pb.yaw = (j as f64) * yaw_radians_factor;
                pb.scale = common_scale;
                pb.into_matrix() // build the projection matrix
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
        }
    }

    let image_path = format!("images/{}_yaw_pitch.svg", program_name);
    let drawing_area = SVGBackend::new(&image_path, image_size).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let areas = drawing_area
        .split_evenly((i_divisions, 1))
        .iter()
        .map(|area| area.split_evenly((1, j_divisions)))
        .collect::<Vec<_>>();

    let pitch_radians_factor = match j_divisions {
        2 => std::f64::consts::PI,
        4 => std::f64::consts::FRAC_PI_2,
        8 => std::f64::consts::FRAC_PI_4,
        16 => std::f64::consts::FRAC_PI_8,
        _ => std::f64::consts::FRAC_PI_8,
    };
    let yaw_radians_factor = match j_divisions {
        2 => std::f64::consts::PI,
        4 => std::f64::consts::FRAC_PI_2,
        8 => std::f64::consts::FRAC_PI_4,
        16 => std::f64::consts::FRAC_PI_8,
        _ => std::f64::consts::FRAC_PI_8,
    };
    for i in 0..areas.len() {
        for j in 0..areas[0].len() {
            let (caption_area, drawing_area) =
                &areas[i][j].split_vertically(caption_area_height as u32);

            // Caption area shows the fraction of π for pitch and yaw
            let fraction_of_i_pi = Fraction::from(i) / Fraction::from(i_divisions / 2);
            let fraction_of_j_pi = Fraction::from(j) / Fraction::from(j_divisions / 2);
            let (pi_str_i, pi_str_j) =
                (if i == 0 { "" } else { "π" }, if j == 0 { "" } else { "π" });
            caption_area.titled(
                &format!("y:{fraction_of_i_pi}{pi_str_i}, p:{fraction_of_j_pi}{pi_str_j}"),
                &caption_style,
            )?;

            // Drawing area
            let mut cc = ChartBuilder::on(&drawing_area)
                .margin(common_margin)
                .margin_top(margin_top)
                .margin_bottom(margin_bottom)
                .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

            cc.with_projection(|mut pb| {
                pb.pitch = 0.;
                pb.yaw = (i as f64) * yaw_radians_factor;
                pb.pitch = (j as f64) * pitch_radians_factor;
                pb.scale = common_scale;
                pb.into_matrix() // build the projection matrix
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
        }
    }

    Ok(())
}
