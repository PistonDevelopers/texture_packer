use {
    Buffer2d,
    Rect,
};

pub trait Packer {
    type Buffer: Buffer2d;

    fn new(b: Self::Buffer) -> Self;
    fn buf(&self) -> &Self::Buffer;
    fn pack<O: Buffer2d<Pixel=<Self::Buffer as Buffer2d>::Pixel>>(&mut self, buf: &O) -> Option<Rect>;
    fn set_margin(&mut self, _val: u32) {}
    fn into_buf(self) -> Self::Buffer;
}

