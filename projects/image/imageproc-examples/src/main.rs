use std::env;
use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

fn main() {
    let arg = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a target file path")
    };

    let path = Path::new(&arg);
    let blue  = Rgb([0u8, 0u8, 255u8]);
    let mut image = RgbImage::new(200, 200);
    draw_filled_circle_mut(&mut image, (150, 100), 15, blue);
    image.save(path).unwrap();
}
