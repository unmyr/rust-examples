use image::{self, ImageBuffer};

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;

    // Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(
        width, height,
        |x, y| {
            image::Luma([((x/2 + y/2) / 2) as u8])
        }
    );

    // Write the contents of this image to the Writer in PNG format.
    img.save("images/x_plus_y.png").unwrap();

    // Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(
        width, height,
        |x, y| {
            image::Luma([((x * y) / 2) as u8])
        }
    );

    // Write the contents of this image to the Writer in PNG format.
    img.save("images/x_times_y.png").unwrap();

    // Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(
        width, height,
        |x, y| {
            image::Luma([(x/2 ^ y/2) as u8])
        }
    );

    // Write the contents of this image to the Writer in PNG format.
    img.save("images/x_xor_y.png").unwrap();
}
