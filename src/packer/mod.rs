use crate::{
    frame::Frame,
    texture::{Pixel, Texture},
};

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer {
    type Pixel: Pixel;

    fn pack(&mut self, key: String, texture: &dyn Texture<Pixel = Self::Pixel>) -> Option<Frame>;
    fn can_pack(&self, texture: &dyn Texture<Pixel = Self::Pixel>) -> bool;
}
