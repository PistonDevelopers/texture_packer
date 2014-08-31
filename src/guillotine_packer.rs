
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

pub struct GuillotinePacker {
    buf: DynamicImage,
    free_areas: Vec<Rect>,
}

impl GuillotinePacker {
    pub fn new(width: u32, height: u32) -> GuillotinePacker {
        let mut free_areas = Vec::new();
        free_areas.push(Rect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        });

        GuillotinePacker {
            buf: ImageRgba8(ImageBuf::new(width, height)),
            free_areas: free_areas,
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

impl Packer for GuillotinePacker {
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

