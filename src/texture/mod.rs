pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait Texture {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn get(&self, x: u32, y: u32) -> Pixel;
    fn set(&mut self, x: u32, y: u32, val: Pixel);

    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}
