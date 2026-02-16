use std::env;

use plotters::element::Drawable;
use plotters::element::PointCollection;
use plotters::prelude::*;
use plotters::style::SizeDesc;
use plotters_backend::BackendCoord;
use plotters_backend::DrawingErrorKind;

pub struct SquareMarker<Coord, Size: SizeDesc> {
    center: Coord,
    size: Size,
    style: ShapeStyle,
}

impl<Coord, Size: SizeDesc> SquareMarker<Coord, Size> {
    // Creates a square marker
    pub fn new<T: Into<ShapeStyle>>(coord: Coord, size: Size, style: T) -> Self {
        Self {
            center: coord,
            size,
            style: style.into(),
        }
    }
}

impl<'a, Coord: 'a, Size: SizeDesc> PointCollection<'a, Coord> for &'a SquareMarker<Coord, Size> {
    type Point = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> std::iter::Once<&'a Coord> {
        std::iter::once(&self.center)
    }
}

impl<Coord, DB: DrawingBackend, Size: SizeDesc> Drawable<DB> for SquareMarker<Coord, Size> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        ps: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            let size = self.size.in_pixels(&ps);
            let (x0, y0) = (x - size, y - size);
            let (x1, y1) = (x + size, y + size);
            backend.draw_line((x0, y0), (x1, y0), &self.style)?;
            backend.draw_line((x1, y0), (x1, y1), &self.style)?;
            backend.draw_line((x1, y1), (x0, y1), &self.style)?;
            backend.draw_line((x0, y1), (x0, y0), &self.style)?;
        }
        Ok(())
    }
}

// Draw a graph of the inverse sigmoid function (logit)
fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "plotters_points_element_markers",
    };

    let image_path_buf = std::path::PathBuf::from("../images").join(format!("{program_name}.png"));

    // Create root drawing area
    let root_area = BitMapBackend::new(&image_path_buf, (300, 320)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // Set x-axis
    let (x_min, x_max): (f32, f32) = (-0.05_f32, 1.05_f32);
    let (y_min, y_max): (f32, f32) = (-0.05_f32, 1.05_f32);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Custom element markers", ("sans-serif", 24))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    cc.configure_mesh()
        .x_labels(5)
        .y_labels(3)
        .max_light_lines(4)
        .draw()
        .unwrap();

    cc.draw_series(PointSeries::of_element(
        vec![(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)],
        5,
        &RED,
        &|p, size, style| {
            if p.0 == 0. && p.1 == 0. {
                Cross::new(p, size, style).into_dyn()
            } else if p.0 == 0. && p.1 == 1. {
                TriangleMarker::new(p, size, style.filled()).into_dyn()
            } else if p.0 == 1. && p.1 == 0. {
                SquareMarker::new(p, size, style).into_dyn()
            } else {
                Circle::new(p, size, style).into_dyn()
            }
        },
    ))
    .unwrap();
}
