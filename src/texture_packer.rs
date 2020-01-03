use crate::{
    frame::Frame,
    packer::{Packer, SkylinePacker},
    rect::Rect,
    texture::{Pixel, SubTexture, Texture},
    texture_packer_config::TexturePackerConfig,
};
use std::collections::HashMap;

pub type PackResult<T> = Result<T, PackError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackError {
    TextureTooLargeToFitIntoAtlas,
}

pub struct TexturePacker<'a, T: 'a + Clone, P> {
    textures: HashMap<String, SubTexture<'a, T>>,
    frames: HashMap<String, Frame>,
    packer: P,
    config: TexturePackerConfig,
}

impl<'a, Pix: Pixel, T: 'a + Clone + Texture<Pixel = Pix>>
    TexturePacker<'a, T, SkylinePacker<Pix>>
{
    pub fn new_skyline(config: TexturePackerConfig) -> TexturePacker<'a, T, SkylinePacker<Pix>> {
        TexturePacker {
            textures: HashMap::new(),
            frames: HashMap::new(),
            packer: SkylinePacker::<Pix>::new(config),
            config: config,
        }
    }
}

impl<'a, Pix: Pixel, P: Packer<Pixel = Pix>, T: Clone + Texture<Pixel = Pix>>
    TexturePacker<'a, T, P>
{
    pub fn can_pack(&self, texture: &'a T) -> bool {
        self.packer.can_pack(texture)
    }

    pub fn pack_ref(&mut self, key: String, texture: &'a T) -> PackResult<()> {
        if !self.packer.can_pack(texture) {
            return Err(PackError::TextureTooLargeToFitIntoAtlas);
        }

        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::from_ref(texture, source);
        if let Some(mut frame) = self.packer.pack(key.clone(), &texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            frame.trimmed = self.config.trim;
            frame.source = source;
            frame.source.w = w;
            frame.source.h = h;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, texture);
        Ok(())
    }

    pub fn pack_own(&mut self, key: String, texture: T) -> PackResult<()> {
        if !self.packer.can_pack(&texture) {
            return Err(PackError::TextureTooLargeToFitIntoAtlas);
        }

        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(&texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::new(texture, source);
        if let Some(mut frame) = self.packer.pack(key.clone(), &texture) {
            frame.frame.x += self.config.border_padding;
            frame.frame.y += self.config.border_padding;
            frame.trimmed = self.config.trim;
            frame.source = source;
            frame.source.w = w;
            frame.source.h = h;
            self.frames.insert(key.clone(), frame);
        }

        self.textures.insert(key, texture);
        Ok(())
    }

    pub fn get_frames(&self) -> &HashMap<String, Frame> {
        &self.frames
    }

    pub fn get_frame(&self, key: &str) -> Option<&Frame> {
        if let Some(frame) = self.frames.get(key) {
            Some(frame)
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

impl<'a, Pix, P, T: Clone> Texture for TexturePacker<'a, T, P>
where
    Pix: Pixel,
    P: Packer<Pixel = Pix>,
    T: Texture<Pixel = Pix>,
{
    type Pixel = Pix;

    fn width(&self) -> u32 {
        let mut right = None;

        for (_, frame) in self.frames.iter() {
            if let Some(r) = right {
                if frame.frame.right() > r {
                    right = Some(frame.frame.right());
                }
            } else {
                right = Some(frame.frame.right());
            }
        }

        if let Some(right) = right {
            right + 1 + self.config.border_padding
        } else {
            0
        }
    }

    fn height(&self) -> u32 {
        let mut bottom = None;

        for (_, frame) in self.frames.iter() {
            if let Some(b) = bottom {
                if frame.frame.bottom() > b {
                    bottom = Some(frame.frame.bottom());
                }
            } else {
                bottom = Some(frame.frame.bottom());
            }
        }

        if let Some(bottom) = bottom {
            bottom + 1 + self.config.border_padding
        } else {
            0
        }
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

fn trim_texture<T: Texture>(texture: &T) -> Rect {
    let mut x1 = 0;
    for x in 0..texture.width() {
        if texture.is_column_transparent(x) {
            x1 = x + 1;
        } else {
            break;
        }
    }

    let mut x2 = texture.width() - 1;
    for x in 0..texture.width() {
        let x = texture.width() - x - 1;
        if texture.is_column_transparent(x) {
            x2 = x - 1;
        } else {
            break;
        }
    }

    let mut y1 = 0;
    for y in 0..texture.height() {
        if texture.is_row_transparent(y) {
            y1 = y + 1;
        } else {
            break;
        }
    }

    let mut y2 = texture.height() - 1;
    for y in 0..texture.height() {
        let y = texture.height() - y - 1;
        if texture.is_row_transparent(y) {
            y2 = y - 1;
        } else {
            break;
        }
    }
    Rect::new_with_points(x1, y1, x2, y2)
}
