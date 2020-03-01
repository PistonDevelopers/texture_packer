//! Defines an RGBA8-based texture and pixel format.
use crate::texture::{Pixel, Texture};

/// [Pixel] format for [MemoryRGBA8Texture].
#[derive(Copy, Clone)]
pub struct RGBA8 {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha component.
    pub a: u8,
}

impl Pixel for RGBA8 {
    fn is_transparent(&self) -> bool {
        self.a == 0
    }

    fn transparency() -> Option<Self> {
        Some(RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
    }

    fn outline() -> RGBA8 {
        RGBA8 {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

/// Texture from RGBA8 pixel data.
#[derive(Clone)]
pub struct MemoryRGBA8Texture {
    pixels: Vec<RGBA8>,
    width: u32,
    height: u32,
}

impl MemoryRGBA8Texture {
    /// Create a texture from memory given a raw buffer.
    pub fn from_memory(buf: &[u8], w: u32, h: u32) -> MemoryRGBA8Texture {
        let mut pixels = Vec::new();

        assert_eq!((buf.len() / 4) as u32, w * h, "buffer does not contain as many pixels ({}) as specified by the width and height ({}, {}) = {}", buf.len() / 4, w, h, w * h);

        for pixel in buf.chunks(4) {
            pixels.push(RGBA8 {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }

        MemoryRGBA8Texture {
            pixels,
            width: w,
            height: h,
        }
    }

    #[inline(always)]
    fn index_for(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}

impl Texture for MemoryRGBA8Texture {
    type Pixel = RGBA8;

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn get(&self, x: u32, y: u32) -> Option<RGBA8> {
        if let Some(p) = self.pixels.get(self.index_for(x, y)) {
            Some(*p)
        } else {
            None
        }
    }

    fn set(&mut self, x: u32, y: u32, val: RGBA8) {
        let index = self.index_for(x, y);
        self.pixels[index] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "buffer does not contain as many pixels (0) as specified by the width and height (1, 1) = 1"
    )]
    fn input_data_not_divisible_by_4() {
        MemoryRGBA8Texture::from_memory(&[0], 1, 1);
    }
}
