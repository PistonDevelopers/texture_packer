
use {
    Buffer2d,
    Rect,
    Packer,
};

pub struct GuillotinePacker<'a> {
    buf: &'a mut Buffer2d,
    free_areas: Vec<Rect>,
    margin: u32,
}

impl<'a> GuillotinePacker<'a> {
    pub fn new(buf: &'a mut Buffer2d) -> GuillotinePacker<'a> {
        let (width, height) = buf.dimensions();
        let mut free_areas = Vec::new();
        free_areas.push(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });

        GuillotinePacker {
            buf: buf,
            free_areas: free_areas,
            margin: 0,
        }
    }

    // Best Area Fit
    fn find_free_area(&self, w: u32, h: u32) -> Option<(uint, Rect)> {
        let mut index = None;
        let mut min_area = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        for i in range(0, self.free_areas.len()) {
            let ref area = self.free_areas[i];
            let a = area.area();

            if w <= area.w && h <= area.h {
                if min_area.is_none() || a < min_area.unwrap() {
                    index = Some(i);
                    min_area = Some(a);
                    rect.x = area.x;
                    rect.y = area.y;
                    rect.w = w;
                    rect.h = h;
                }
            } else if h <= area.w && w <= area.h {
                if min_area.is_none() || a < min_area.unwrap() {
                    index = Some(i);
                    min_area = Some(a);
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

    // Shorter Axis Split
    fn split(&mut self, index: uint, rect: &Rect) {
        let area = self.free_areas.remove(index).unwrap();

        // Split horizontally
        if area.w < area.h {
            self.free_areas.push(Rect {
                x: area.x + rect.w,
                y: area.y,
                w: area.w - rect.w,
                h: rect.h,
            });

            self.free_areas.push(Rect {
                x: area.x,
                y: area.y + rect.h,
                w: area.w,
                h: area.h - rect.h,
            });
        }
        // Split vertically
        else {
            self.free_areas.push(Rect {
                x: area.x,
                y: area.y + rect.h,
                w: rect.w,
                h: area.h - rect.h,
            });

            self.free_areas.push(Rect {
                x: area.x + rect.w,
                y: area.y,
                w: area.w - rect.w,
                h: area.h,
            });
        }
    }
}

impl<'a> Packer for GuillotinePacker<'a> {
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect> {
        let (mut width, mut height) = buf.dimensions();
        width += self.margin;
        height += self.margin;
        match self.find_free_area(width, height) {
            Some((i, rect)) => {
                if width == rect.w {
                    self.buf.patch(rect.x, rect.y, buf);
                } else {
                    self.buf.patch_rotated(rect.x, rect.y, buf);
                }

                self.split(i, &rect);

                Some(rect)
            },
            _ => {
                None
            },
        }
    }

    fn buf(&self) -> &Buffer2d {
        self.buf
    }

    fn set_margin(&mut self, val: u32) {
        self.margin = val;
    }
}

