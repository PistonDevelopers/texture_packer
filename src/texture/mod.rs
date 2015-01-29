pub use self::memory_rgba8_texture::MemoryRGBA8Texture;

pub mod memory_rgba8_texture;
pub mod image_texture;

pub trait Texture {
    type Pixel: Pixel;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    // TODO: Chanage returen value to &-ptr
    fn get(&self, x: u32, y: u32) -> Option<Self::Pixel>;
    fn set(&mut self, x: u32, y: u32, val: Self::Pixel);

    fn get_rotated(&self, x: u32, y: u32) -> Option<Self::Pixel> {
        let w = self.height();
        self.get(y, w - x - 1)
    }
}

pub trait Pixel {
    fn is_transparent(&self) -> bool;
    fn transparency() -> Option<Self>;
    fn outline() -> Self;
}
