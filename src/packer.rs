
use {
    Buffer2d,
    Rect,
};

pub trait Packer<B: Buffer2d> {
    fn buf(&self) -> &B;
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect>;
    fn set_margin(&mut self, _val: u32) {}
}

