
use packer::{
    Packer,
    patch,
    patch_rotated,
};

use rect::Rect;

use image::{
    ImageRgba8,
    DynamicImage,
    GenericImage,
    ImageBuf,
};

pub struct MaxrectPacker {
    buf: DynamicImage,
    free_areas: Vec<Rect>,
}

impl MaxrectPacker {
    pub fn new(width: u32, height: u32) -> MaxrectPacker {
        let mut free_areas = Vec::new();
        free_areas.push(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });

        MaxrectPacker {
            buf: ImageRgba8(ImageBuf::new(width, height)),
            free_areas: free_areas,
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
    }

    fn divide(&mut self, rect: &Rect) {
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

impl Packer for MaxrectPacker {
    fn pack(&mut self, image: &DynamicImage) -> Option<Rect> {
        let (image_width, image_height) = image.dimensions();
        match self.find_free_area(image_width, image_height) {
            Some((i, rect)) => {
                if image_width == rect.w {
                    patch(&mut self.buf, rect.x, rect.y, image);
                } else {
                    patch_rotated(&mut self.buf, rect.x, rect.y, image);
                }

                self.split(i, &rect);
                self.divide(&rect);
                self.merge();

                Some(rect)
            },
            _ => {
                None
            },
        }
    }

    fn image(&self) -> &DynamicImage {
        &self.buf
    }
}
