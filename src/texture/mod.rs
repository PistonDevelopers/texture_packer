pub use self::{memory_rgba8_texture::MemoryRGBA8Texture, sub_texture::SubTexture};
use std::ops::{Deref, DerefMut};

pub mod image_texture;
pub mod memory_rgba8_texture;
pub mod sub_texture;

pub trait Texture {
    type Pixel: Pixel;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    // TODO: Chanage return value to &-ptr
    fn get(&self, x: u32, y: u32) -> Option<Self::Pixel>;
    fn set(&mut self, x: u32, y: u32, val: Self::Pixel);

    fn get_rotated(&self, x: u32, y: u32) -> Option<Self::Pixel> {
        let w = self.height();
        self.get(y, w - x - 1)
    }

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

pub trait Pixel: Sized {
    fn is_transparent(&self) -> bool;
    fn transparency() -> Option<Self>;
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
