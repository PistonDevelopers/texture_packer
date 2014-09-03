
use {
    Buffer2d,
    Packer,
    Rect,
};

pub struct MaxrectPacker<'a, B: 'a + Buffer2d> {
    buf: B,
    free_areas: Vec<Rect>,
    margin: u32,
}

impl<'a, B: Buffer2d> MaxrectPacker<'a, B> {
    pub fn new(mut buf: B) -> MaxrectPacker<'a, B> {
        let (width, height) = buf.dimensions();

        let mut free_areas = Vec::new();
        free_areas.push(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });

        MaxrectPacker {
            buf: buf,
            free_areas: free_areas,
            margin: 0,
        }
    }

    fn find_free_area(&self, w: u32, h: u32) -> Option<(uint, Rect)> {
        let mut min_x = None;
        let mut min_y = None;
        let mut index = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        for i in range(0, self.free_areas.len()) {
            let ref area = self.free_areas[i];

            if w <= area.w && h <= area.h {
                if min_y.is_none() || area.y < min_y.unwrap() ||
                   (area.y == min_y.unwrap() && area.x < min_x.unwrap()) {
                    index = Some(i);
                    min_x = Some(area.x);
                    min_y = Some(area.y);
                    rect.x = area.x;
                    rect.y = area.y;
                    rect.w = w;
                    rect.h = h;
                }
            } else if h <= area.w && w <= area.h {
                if min_y.is_none() || area.y < min_y.unwrap() ||
                   (area.y == min_y.unwrap() && area.x < min_x.unwrap()) {
                    index = Some(i);
                    min_x = Some(area.x);
                    min_y = Some(area.y);
                    rect.x = area.x;
                    rect.y = area.y;
                    rect.w = h;
                    rect.h = w;
                }
            }
        }

        match index {
            Some(i) => {
                Some((i, rect))
            },
            _ => {
                None
            },
        }
    }

    fn split(&mut self, index: uint, rect: &Rect) {
        let area = self.free_areas.remove(index).unwrap();
        self.free_areas.push(Rect {
            x: area.x + rect.w,
            y: area.y,
            w: area.w - rect.w,
            h: area.h,
        });

        self.free_areas.push(Rect {
            x: area.x,
            y: area.y + rect.h,
            w: area.w,
            h: area.h - rect.h,
        });

        let mut new_free_areas = Vec::new();
        for free_area in self.free_areas.iter() {
            new_free_areas = new_free_areas.append(free_area.crop(rect).as_slice());
        }
        self.free_areas = new_free_areas;
    }

    fn merge(&mut self) {
        if self.free_areas.len() > 1 {
            let mut new_free_areas = Vec::new();
            let mut to_be_removed = Vec::new();

            for i in range(0, self.free_areas.len()) {
                for j in range(0, self.free_areas.len()) {
                    if i != j {
                        if self.free_areas[i].contains(&self.free_areas[j]) {
                            to_be_removed.push(j);
                        }
                    }
                }
            }

            for i in range(0, self.free_areas.len()) {
                if !to_be_removed.contains(&i) {
                    new_free_areas.push(self.free_areas[i]);
                }
            }
            self.free_areas = new_free_areas;
        }
    }
}

impl<'a, B: Buffer2d> Packer<B> for MaxrectPacker<'a, B> {
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect> {
        let (mut width, mut height) = buf.dimensions();
        width += self.margin;
        height += self.margin;
        match self.find_free_area(width, height) {
            Some((i, mut rect)) => {
                if width == rect.w {
                    self.buf.patch(rect.x, rect.y, buf);
                } else {
                    self.buf.patch_rotated(rect.x, rect.y, buf);
                }

                self.split(i, &rect);
                self.merge();

                rect.w -= self.margin;
                rect.h -= self.margin;
                Some(rect)
            },
            _ => {
                None
            },
        }
    }

    fn buf(&self) -> &B {
        &self.buf
    }

    fn set_margin(&mut self, val: u32) {
        self.margin = val;
    }
}
