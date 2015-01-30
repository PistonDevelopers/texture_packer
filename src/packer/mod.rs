use texture::{
    Pixel,
    Texture,
};
use frame::Frame;

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer {
    type Pixel: Pixel;

    fn pack(&mut self, key: String, texture: &Texture<Pixel = Self::Pixel>) -> Option<Frame>;
}
