use image::GenericImage;

pub trait Buffer2d: Sized {
    type Pixel;

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn get(&self, x: u32, y: u32) -> Option<Self::Pixel>;
    fn set(&mut self, x: u32, y: u32, _val: Self::Pixel);

    fn patch<B: Buffer2d<Pixel=Self::Pixel>>
    (&mut self, x: u32, y: u32, buf: &B) {
        let (w, h) = buf.dimensions();

        for sy in range(0, h) {
            for sx in range(0, w) {

                match buf.get(sx, sy) {
                    Some(val) => {
                        self.set(x + sx, y + sy, val);
                    },
                    _ => {},
                }
            }
        }
    }

    fn patch_rotated<B: Buffer2d<Pixel=Self::Pixel>>
    (&mut self, x: u32, y: u32, buf: &B) {
        let (w, h) = buf.dimensions();

        for sy in range(0, h) {
            for sx in range(0, w) {
                match buf.get(sx, sy) {
                    Some(val) => {
                        self.set(x + h - sy - 1, y + sx, val);
                    },
                    _ => {},
                }
            }
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}

impl <G: GenericImage> Buffer2d for G {
    type Pixel = G::Pixel;

    fn width(&self) -> u32 {
        GenericImage::dimensions(self).0
    }

    fn height(&self) -> u32 {
        self.dimensions().1
    }

    fn get(&self, x: u32, y: u32) -> Option<<Self as Buffer2d>::Pixel> {
        Some(self.get_pixel(x, y))
    }

    fn set(&mut self, x: u32, y: u32, val: <Self as Buffer2d>::Pixel) {
        self.put_pixel(x, y, val);
    }
}
