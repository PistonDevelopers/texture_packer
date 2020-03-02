//! Define [Texture] and [Pixel] for a generic [Image](image::GenericImage).
use crate::texture::{Pixel, Texture};
use image::{GenericImage, Rgb, Rgba};

impl<P: Pixel + image::Pixel, I: GenericImage<Pixel = P>> Texture for I {
    type Pixel = I::Pixel;

    fn width(&self) -> u32 {
        self.dimensions().0
    }

    fn height(&self) -> u32 {
        self.dimensions().1
    }

    fn get(&self, x: u32, y: u32) -> Option<I::Pixel> {
        if self.in_bounds(x, y) {
            Some(self.get_pixel(x, y))
        } else {
            None
        }
    }

    fn set(&mut self, x: u32, y: u32, val: I::Pixel) {
        self.put_pixel(x, y, val);
    }
}

impl Pixel for Rgba<u8> {
    fn is_transparent(&self) -> bool {
        self[3] == 0
    }

    fn transparency() -> Option<Rgba<u8>> {
        Some(Rgba([0; 4]))
    }

    fn outline() -> Rgba<u8> {
        Rgba([255, 0, 0, 255])
    }
}

impl Pixel for Rgb<u8> {
    fn is_transparent(&self) -> bool {
        false
    }

    fn transparency() -> Option<Rgb<u8>> {
        None
    }

    fn outline() -> Rgb<u8> {
        Rgb([255, 0, 0])
    }
}
