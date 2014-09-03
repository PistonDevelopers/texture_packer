
use image;
use image::{
    ImageBuf,
    ImageRgba8,
    ImageRgb8,
    ImageLuma8,
    DynamicImage,
    GenericImage,
    Pixel,
};

use {
    ColorType,
    RGBA,
    RGB,
    Grey,

    Color,
    RGBA8,
    RGB8,
    Grey8,

    Buffer2d,
};

pub struct ImageBuffer {
    image: DynamicImage,
}

impl ImageBuffer {
    pub fn new(w: u32, h: u32, color_type: ColorType) -> ImageBuffer {
        ImageBuffer {
            image: match color_type {
                RGBA => {
                   ImageRgba8(ImageBuf::new(w, h))
                },
                RGB => {
                    ImageRgb8(ImageBuf::new(w, h))
                },
                Grey => {
                    ImageLuma8(ImageBuf::new(w, h))
                },
            },
        }
    }

    pub fn open(path: &Path) -> Option<ImageBuffer> {
        match image::open(path) {
            Ok(dynimage) => {
                Some(ImageBuffer {
                    image: dynimage,
                })
            },
            Err(_) => {
                None
            },
        }
    }

    pub fn image(&self) -> &DynamicImage {
        &self.image
    }
}

impl Buffer2d for ImageBuffer {
    fn width(&self) -> u32 {
        match self.image.dimensions() {
            (w, _) => { w },
        }
    }

    fn height(&self) -> u32 {
        match self.image.dimensions() {
            (_, h) => { h },
        }
    }

    fn get(&self, x: u32, y: u32) -> Option<Color> {
        Some(RGBA8(self.image.get_pixel(x, y)))
    }

    fn set(&mut self, x: u32, y: u32, val: Color) {
        match val {
            RGBA8(val) => {
                self.image.put_pixel(x, y, val);
            },
            RGB8(val) => {
                self.image.put_pixel(x, y, val.to_rgba());
            },
            Grey8(val) => {
                self.image.put_pixel(x, y, val.to_rgba());
            },
        }
    }
}

