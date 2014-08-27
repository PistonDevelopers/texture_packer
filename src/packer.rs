
use image::{
    DynamicImage,
    GenericImage,
};

pub trait Packer {
    fn image(&self) -> &DynamicImage;
    fn pack(&mut self, image: &DynamicImage);
}

pub fn patch(buf: &mut DynamicImage, x: u32, y: u32, image: &DynamicImage) {
    let (image_width, image_height) = image.dimensions();

    for sy in range(0, image_height) {
        for sx in range(0, image_width) {
            buf.put_pixel(x + sx, y + sy, image.get_pixel(sx, sy));
        }
    }
}

pub fn patch_rotated(buf: &mut DynamicImage, x: u32, y: u32, image: &DynamicImage) {
    let (image_width, image_height) = image.dimensions();
    let (buf_width, buf_height) = buf.dimensions();

    for sy in range(0, image_height) {
        for sx in range(0, image_width) {
            let dx = x + image_height - sy - 1;
            let dy = y + sx;
            if dx < buf_width && dy < buf_height {
                buf.put_pixel(dx, dy, image.get_pixel(sx, sy));
            } else {
                println!("{}, {}", dx, dy);
            }
        }
    }
}

