
use image;
use image::{
    ImageBuffer,
    ImageRgba8,
    ImageRgb8,
    ImageLuma8,
    DynamicImage,
    GenericImage,
    Pixel,
};

use {
    ColorType,
    Color,
    Buffer2d,
};

pub struct ImgBuffer {
    image: DynamicImage,
}

impl ImgBuffer {
    pub fn new(w: u32, h: u32, color_type: ColorType) -> ImgBuffer {
        ImgBuffer {
            image: match color_type {
                ColorType::RGBA => {
                   ImageRgba8(ImageBuffer::new(w, h))
                },
                ColorType::RGB => {
                    ImageRgb8(ImageBuffer::new(w, h))
                },
                ColorType::Grey => {
                    ImageLuma8(ImageBuffer::new(w, h))
                },
            },
        }
    }

    pub fn open(path: &Path) -> Option<ImgBuffer> {
        match image::open(path) {
            Ok(dynimage) => {
                Some(ImgBuffer {
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

impl Buffer2d for ImgBuffer {
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
        Some(Color::RGBA8(self.image.get_pixel(x, y)))
    }

    fn set(&mut self, x: u32, y: u32, val: Color) {
        match val {
            Color::RGBA8(val) => {
                self.image.put_pixel(x, y, val);
            },
            Color::RGB8(val) => {
                self.image.put_pixel(x, y, val.to_rgba());
            },
            Color::Grey8(val) => {
                self.image.put_pixel(x, y, val.to_rgba());
            },
        }
    }
}

