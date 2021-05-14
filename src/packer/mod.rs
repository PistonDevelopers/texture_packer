use crate::{frame::Frame, rect::Rect};

pub use self::skyline_packer::SkylinePacker;

mod skyline_packer;

pub trait Packer<K> {
    fn pack(&mut self, key: K, texture_rect: &Rect) -> Option<Frame<K>>;
    fn can_pack(&self, texture_rect: &Rect) -> bool;
}
