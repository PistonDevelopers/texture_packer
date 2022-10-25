//! Define [Texture] and [Pixel] for a generic [Image](image::GenericImage).
use crate::texture::{Pixel, Texture};
use image::{GenericImage, Primitive, Rgb, Rgba};

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

impl<T: Primitive> Pixel for Rgb<T> {
    fn is_transparent(&self) -> bool {
        false
    }

    fn transparency() -> Option<Rgb<T>> {
        None
    }

    fn outline() -> Rgb<T> {
        Rgb([
            T::DEFAULT_MAX_VALUE,
            T::DEFAULT_MIN_VALUE,
            T::DEFAULT_MIN_VALUE,
        ])
    }
}

impl<T: Primitive> Pixel for Rgba<T> {
    fn is_transparent(&self) -> bool {
        self[3] == T::DEFAULT_MIN_VALUE
    }

    fn transparency() -> Option<Rgba<T>> {
        Some(Rgba([T::DEFAULT_MIN_VALUE; 4]))
    }

    fn outline() -> Rgba<T> {
        Rgba([
            T::DEFAULT_MAX_VALUE,
            T::DEFAULT_MIN_VALUE,
            T::DEFAULT_MIN_VALUE,
            T::DEFAULT_MAX_VALUE,
        ])
    }
}
