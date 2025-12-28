use plotters::prelude::*;

fn xor_continuous(x1: f64, x2: f64) -> f64 {
    x1 + x2 - 2.0 * x1 * x2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif("images/xor_surface.gif", (600, 400), 100)?.into_drawing_area();
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
    Ok(())
}
