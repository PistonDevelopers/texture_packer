use std::collections::HashMap;

use {
    TexturePackerConfig,
};
use super::Cow;

use texture::{
    Pixel,
    Texture,
};

use frame::Frame;
use packer::{
    Packer,
    SkylinePacker,
};

pub struct TexturePacker<'a, T:  'a, P> {
    textures: HashMap<String, Cow<'a, T>>,
    frames: HashMap<String, Frame>,
    packer: P,
    config: TexturePackerConfig,
}

impl <'a, Pix: Pixel, T: 'a +  Texture<Pixel=Pix>> TexturePacker<'a, T, SkylinePacker<Pix>> {
    pub fn new_skyline(config: TexturePackerConfig) -> TexturePacker<'a, T, SkylinePacker<Pix>> {
        TexturePacker {
            textures: HashMap::new(),
            frames: HashMap::new(),
            packer: SkylinePacker::<Pix>::new(config),
            config: config,
        }
    }
}

impl<'a, Pix: Pixel, P: Packer<Pixel=Pix>, T:  Texture<Pixel=Pix>> TexturePacker<'a, T, P> {
    pub fn pack_ref(&mut self, key: String, texture: &'a T) {
        if let Some(mut frame) = self.packer.pack(key.clone(), texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, Cow::Borrowed(texture));
    }

    pub fn pack_own(&mut self, key: String, texture: T) {
        if let Some(mut frame) = self.packer.pack(key.clone(), &texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, Cow::Owned(texture));
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

impl<'a, Pix, P, T> Texture for  TexturePacker<'a, T, P>
where Pix: Pixel, P: Packer<Pixel=Pix>, T:  Texture<Pixel=Pix> {
    type Pixel = Pix;

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

    fn get(&self, x: u32, y: u32) -> Option<Pix> {
        if let Some(frame) = self.get_frame_at(x, y) {
            if let Some(texture) = self.textures.get(&frame.key) {
                if self.config.texture_outlines && frame.frame.is_outline(x, y) {
                    return Some(<Pix as Pixel>::outline());
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

    fn set(&mut self, _x: u32, _y: u32, _val: Pix) {
        panic!("Can't set pixel directly");
    }
}
