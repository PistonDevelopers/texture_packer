use crate::{
    frame::Frame,
    packer::{Packer, SkylinePacker},
    rect::Rect,
    texture::{Pixel, SubTexture, Texture},
    texture_packer_config::TexturePackerConfig,
};
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;

pub type PackResult<T> = Result<T, PackError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackError {
    TextureTooLargeToFitIntoAtlas,
}

/// Packs textures into a single texture atlas.
pub struct TexturePacker<'a, T: 'a + Clone, K: Clone + Eq + Hash> {
    textures: HashMap<K, SubTexture<'a, T>>,
    frames: HashMap<K, Frame<K>>,
    packer: Box<dyn Packer<K>>,
    config: TexturePackerConfig,
}

impl<'a, Pix: Pixel, T: 'a + Clone + Texture<Pixel = Pix>, K: Clone + Eq + Hash>
    TexturePacker<'a, T, K>
{
    /// Create a new packer using the skyline packing algorithm.
    pub fn new_skyline(config: TexturePackerConfig) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            frames: HashMap::new(),
            packer: Box::new(SkylinePacker::new(config)),
            config,
        }
    }
}

impl<'a, Pix: Pixel, T: Clone + Texture<Pixel = Pix>, K: Clone + Eq + Hash>
    TexturePacker<'a, T, K>
{
    /// Check if the texture can be packed into this packer.
    pub fn can_pack(&self, texture: &'a T) -> bool {
        let rect = texture.into();
        self.packer.can_pack(&rect)
    }

    /// Pack the `texture` into this packer, taking a reference of the texture object.
    pub fn pack_ref(&mut self, key: K, texture: &'a T) -> PackResult<()> {
        let rect = texture.into();
        if !self.packer.can_pack(&rect) {
            return Err(PackError::TextureTooLargeToFitIntoAtlas);
        }

        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::from_ref(texture, source);
        let rect = (&texture).into();
        if let Some(mut frame) = self.packer.pack(key.clone(), &rect) {
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

    /// Pack the `texture` into this packer, taking ownership of the texture object.
    pub fn pack_own(&mut self, key: K, texture: T) -> PackResult<()> {
        let rect = (&texture).into();
        if !self.packer.can_pack(&rect) {
            return Err(PackError::TextureTooLargeToFitIntoAtlas);
        }

        let (w, h) = (texture.width(), texture.height());
        let source = if self.config.trim {
            trim_texture(&texture)
        } else {
            Rect::new(0, 0, w, h)
        };

        let texture = SubTexture::new(texture, source);
        let rect = (&texture).into();
        if let Some(mut frame) = self.packer.pack(key.clone(), &rect) {
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

    /// Get the backing mapping from strings to frames.
    pub fn get_frames(&self) -> &HashMap<K, Frame<K>> {
        &self.frames
    }

    /// Acquire a frame by its name.
    pub fn get_frame(&self, key: &K) -> Option<&Frame<K>> {
        if let Some(frame) = self.frames.get(key) {
            Some(frame)
        } else {
            None
        }
    }

    /// Get the frame that overlaps with a specified coordinate.
    fn get_frame_at(&self, x: u32, y: u32) -> Option<&Frame<K>> {
        let extrusion = self.config.texture_extrusion;

        for (_, frame) in self.frames.iter() {
            let mut rect = frame.frame;

            rect.x = rect.x.saturating_sub(extrusion);
            rect.y = rect.y.saturating_sub(extrusion);

            rect.w += extrusion * 2;
            rect.h += extrusion * 2;

            if rect.contains_point(x, y) {
                return Some(frame);
            }
        }
        None
    }
}

impl<'a, Pix, T: Clone, K: Clone + Eq + Hash> Texture for TexturePacker<'a, T, K>
where
    Pix: Pixel,
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
            if self.config.texture_outlines && frame.frame.is_outline(x, y) {
                return Some(<Pix as Pixel>::outline());
            }

            if let Some(texture) = self.textures.get(&frame.key) {
                let x = x.saturating_sub(frame.frame.x);
                let y = y.saturating_sub(frame.frame.y);

                let x = min(x, texture.width() - 1);
                let y = min(y, texture.height() - 1);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texture::memory_rgba8_texture::MemoryRGBA8Texture;

    #[test]
    fn able_to_store_in_struct() {
        let packer = TexturePacker::new_skyline(TexturePackerConfig::default());

        struct MyPacker<'a> {
            _packer: TexturePacker<'a, MemoryRGBA8Texture, String>,
        }

        MyPacker { _packer: packer };
    }
}
