use std::collections::HashMap;

use pixel::Pixel;
use texture::Texture;
use frame::Frame;

pub struct TexturePacker<'a> {
    textures: HashMap<String, Box<Texture + 'a>>,
    frames: Vec<Frame>,
}

impl<'a> TexturePacker<'a> {
    pub fn pack(&self) {
        unimplemented!()
    }
}

impl<'a> Texture for TexturePacker<'a> {
    fn width(&self) -> u32 {
        let mut width = 0;

        for frame in self.frames.iter() {
            if frame.frame.right() > width {
                width = frame.frame.right();
            }
        }

        width
    }

    fn height(&self) -> u32 {
        let mut height = 0;

        for frame in self.frames.iter() {
            if frame.frame.bottom() > height {
                height = frame.frame.bottom();
            }
        }

        height
    }

    fn get(&self, x: u32, y: u32) -> Pixel {
        for frame in self.frames.iter() {
            if frame.frame.contains_point(x, y) {
               if let texture = self.textures.get(&frame.key) {
                   return texture.get(x - frame.frame.x, y - frame.frame.y);
               }
            }
        }

        Pixel { r: 0, g: 0, b: 0, a: 0 }
    }

    fn set(&mut self, x: u32, y: u32, val: Pixel) {
        unimplemented!()
    }
}
