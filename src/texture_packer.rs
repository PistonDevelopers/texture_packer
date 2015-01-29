use std::collections::HashMap;

use {
    TexturePackerAlrogithm,
    TexturePackerConfig,
};

use texture::{
    Pixel,
    Texture,
};

use frame::Frame;
use packer::{
    Packer,
    SkylinePacker,
};

pub struct TexturePacker<'a, P: Pixel> {
    textures: HashMap<String, Box<Texture<Pixel = P> + 'a>>,
    frames: HashMap<String, Frame>,
    packer: Box<Packer<Pixel = P> + 'a>,
    config: TexturePackerConfig,
}

impl<'a, P: Pixel> TexturePacker<'a, P> {
    pub fn new(config: TexturePackerConfig) -> TexturePacker<'a, P> {
        let packer = match config.algorithm {
            TexturePackerAlrogithm::Skyline => {
                Box::new(SkylinePacker::new(config))
            }
        };

        TexturePacker {
            textures: HashMap::new(),
            frames: HashMap::new(),
            packer: packer,
            config: config,
        }
    }

    pub fn pack(&mut self, key: String, texture: Box<Texture<Pixel = P> + 'a>) {
        if let Some(mut frame) = self.packer.pack(key.clone(), &*texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, texture);
    }

    pub fn get_frames(&self) -> HashMap<String, Frame> {
        self.frames.clone()
    }

    pub fn get_frame(&self, key: &str) -> Option<Frame> {
        if let Some(frame) = self.frames.get(key) {
            Some(frame.clone())
        } else {
            None
        }
    }

    fn get_frame_at(&self, x: u32, y: u32) -> Option<&Frame> {
        for (_, frame) in self.frames.iter() {
            if frame.frame.contains_point(x, y) {
                return Some(frame);
            }
        }
        None
    }
}

impl<'a, P: Pixel> Texture for TexturePacker<'a, P> {
    type Pixel = P;

    fn width(&self) -> u32 {
        let mut right = 0;

        for (_, frame) in self.frames.iter() {
            if frame.frame.right() > right {
                right = frame.frame.right();
            }
        }

        right + 1 + self.config.border_padding
    }

    fn height(&self) -> u32 {
        let mut bottom = 0;

        for (_, frame) in self.frames.iter() {
            if frame.frame.bottom() > bottom {
                bottom = frame.frame.bottom();
            }
        }

        bottom + 1 + self.config.border_padding
    }

    fn get(&self, x: u32, y: u32) -> Option<P> {
        use rect::Rect;
        let rect = Rect::new(0, 0, self.width(), self.height());
        if rect.is_outline(x, y) {
            return Some(<P as Pixel>::outline());
        }

        if let Some(frame) = self.get_frame_at(x, y) {
            if let Some(texture) = self.textures.get(&frame.key) {
                if self.config.texture_outlines && frame.frame.is_outline(x, y) {
                    return Some(<P as Pixel>::outline());
                }

                let x = x - frame.frame.x;
                let y = y - frame.frame.y;

                return if frame.rotated {
                    texture.get_rotated(x, y)
                } else {
                    texture.get(x, y)
                };
            }
        }

        None
    }

    fn set(&mut self, _x: u32, _y: u32, _val: P) {
        panic!("Can't set pixel directly");
    }
}
