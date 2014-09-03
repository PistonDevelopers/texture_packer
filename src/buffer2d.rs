
use Color;

pub trait Buffer2d {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn get(&self, _x: u32, _y: u32) -> Option<Color> { unimplemented!() }
    fn set(&mut self, _x: u32, _y: u32, _val: Color) { unimplemented!() }

    fn patch(&mut self, x: u32, y: u32, buf: &Buffer2d) {
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

    fn patch_rotated(&mut self, x: u32, y: u32, buf: &Buffer2d) {
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

