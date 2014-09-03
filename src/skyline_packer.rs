
use std;
use std::cmp::max;

use {
    Buffer2d,
    Packer,
    Rect,
};

struct Skyline {
    pub x: u32,
    pub y: u32,
    pub w: u32,
}

pub struct SkylinePacker<B: Buffer2d> {
    buf: B,
    width: u32,
    height: u32,
    skylines: Vec<Skyline>,
    margin: u32,
}

impl<B: Buffer2d> SkylinePacker<B> {
    pub fn new(buf: B) -> SkylinePacker<B> {
        let (width, height) = buf.dimensions();
        let mut skylines = Vec::new();
        skylines.push(Skyline {
            x: 0,
            y: 0,
            w: width,
        });

        SkylinePacker {
            buf: buf,
            width: width,
            height: height,
            skylines: skylines,
            margin: 0,
        }
    }

    fn can_put(&self, i: uint, w: u32, h: u32) -> Option<u32> {
        let x = self.skylines[i].x;
        if x + w > self.width {
            return None;
        }
        let mut width_left = w;
        let mut i = i;
        let mut y = self.skylines[i].y;
        loop {
            y = max(y, self.skylines[i].y);
            if y + h > self.height {
                return None;
            }
            if self.skylines[i].w > width_left {
                return Some(y);
            }
            width_left -= self.skylines[i].w;
            i += 1;
            assert!(i < self.skylines.len());
        }
    }

    fn find_skyline(&self, w: u32, h: u32) -> Option<(uint, Rect)> {
        let mut min_height = std::u32::MAX;
        let mut min_width = std::u32::MAX;
        let mut index = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        // keep the min_height as small as possible
        for i in range(0, self.skylines.len()) {
            match self.can_put(i, w, h) {
                Some(y) => {
                    if y + h < min_height ||
                       (y + h == min_height && self.skylines[i].w < min_width) {
                        min_height = y + h;
                        min_width = self.skylines[i].w;
                        index = Some(i);
                        rect.x = self.skylines[i].x;
                        rect.y = y;
                        rect.w = w;
                        rect.h = h;
                    }
                },
                _ => {},
            }

            match self.can_put(i, h, w) {
                Some(y) => {
                    if y + w < min_height ||
                       (y + w == min_height && self.skylines[i].w < min_width) {
                        min_height = y + w;
                        min_width = self.skylines[i].w;
                        index = Some(i);
                        rect.x = self.skylines[i].x;
                        rect.y = y;
                        rect.w = h;
                        rect.h = w;
                    }
                },
                _ => {},
            }
        }

        if index.is_some() {
            Some((index.unwrap(), rect))
        } else {
            None
        }
    }

    fn split(&mut self, index: uint, rect: &Rect) {
        let skyline = Skyline {
            x: rect.x,
            y: rect.y + rect.h,
            w: rect.w,
        };

        assert!(skyline.x + skyline.w <= self.width);
        assert!(skyline.y <= self.height);

        self.skylines.insert(index, skyline);

        let i = index + 1;
        while i < self.skylines.len() {
            assert!(self.skylines[i-1].x <= self.skylines[i].x);

            if self.skylines[i].x < self.skylines[i-1].x + self.skylines[i-1].w {
                let shrink = self.skylines[i-1].x + self.skylines[i-1].w - self.skylines[i].x;
                if self.skylines[i].w <= shrink {
                    self.skylines.remove(i);
                } else {
                    self.skylines.get_mut(i).x += shrink;
                    self.skylines.get_mut(i).w -= shrink;
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn merge(&mut self) {
        let mut i = 1;
        while i < self.skylines.len() {
            if self.skylines[i-1].y == self.skylines[i].y {
                self.skylines.get_mut(i-1).w += self.skylines[i].w;
                self.skylines.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }
}

impl<B: Buffer2d> Packer<B> for SkylinePacker<B> {
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect> {
        let (mut width, mut height) = buf.dimensions();
        width += self.margin;
        height += self.margin;

        match self.find_skyline(width, height) {
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

