
use {
    Buffer2d,
    Rect,
};

pub trait Packer {
    fn buf(&self) -> &Buffer2d;
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect>;

    fn set_margin(&mut self, _val: u32) {}
}

