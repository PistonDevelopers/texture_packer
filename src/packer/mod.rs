use crate::{frame::Frame, rect::Rect};

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer {
    fn pack(&mut self, key: String, texture_rect: &Rect) -> Option<Frame>;
    fn can_pack(&self, texture_rect: &Rect) -> bool;
}
