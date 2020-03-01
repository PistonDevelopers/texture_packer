//! Traits for a texture and its pixel contents.
pub use self::{memory_rgba8_texture::MemoryRGBA8Texture, sub_texture::SubTexture};
use std::ops::{Deref, DerefMut};

pub mod image_texture;
pub mod memory_rgba8_texture;
pub mod sub_texture;

/// Describes a texture type.
pub trait Texture {
    /// Pixel type of this texture.
    type Pixel: Pixel;

    /// Get the width of this texture.
    fn width(&self) -> u32;
    /// Get the height of this texture.
    fn height(&self) -> u32;
    // TODO: Chanage return value to &-ptr
    /// Get the pixel value at a specific coordinate.
    fn get(&self, x: u32, y: u32) -> Option<Self::Pixel>;
    /// Set the pixel value at a specific coordinate.
    fn set(&mut self, x: u32, y: u32, val: Self::Pixel);

    /// Get the pixel if it were transformed by a rotation.
    fn get_rotated(&self, x: u32, y: u32) -> Option<Self::Pixel> {
        let w = self.height();
        self.get(y, w - x - 1)
    }

    /// Check if a column of the texture is transparent.
    fn is_column_transparent(&self, col: u32) -> bool {
        for y in 0..self.height() {
            if let Some(p) = self.get(col, y) {
                if !p.is_transparent() {
                    return false;
                }
            }
        }
        true
    }

    /// Check if a row of the texture is transparent.
    fn is_row_transparent(&self, row: u32) -> bool {
        for x in 0..self.width() {
            if let Some(p) = self.get(x, row) {
                if !p.is_transparent() {
                    return false;
                }
            }
        }
        true
    }
}

/// Describes a pixel type.
pub trait Pixel: Sized {
    /// If the pixel is transparent.
    fn is_transparent(&self) -> bool;
    /// The transparent value for this pixel type.
    fn transparency() -> Option<Self>;
    /// Outline value for this pixel type.
    fn outline() -> Self;
}

impl<P: Pixel> Texture for Box<dyn Texture<Pixel = P> + 'static> {
    type Pixel = P;

    fn width(&self) -> u32 {
        self.deref().width()
    }

    fn height(&self) -> u32 {
        self.deref().height()
    }

    fn get(&self, x: u32, y: u32) -> Option<P> {
        self.deref().get(x, y)
    }

    fn set(&mut self, x: u32, y: u32, val: P) {
        self.deref_mut().set(x, y, val);
    }
}
