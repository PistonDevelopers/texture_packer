
use {
    Buffer2d,
    Rect,
};

pub trait Packer {
    fn buf(&self) -> &Buffer2d;
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect>;
}

