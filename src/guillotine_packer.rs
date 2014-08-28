
use packer::{
    Packer,
    patch,
    patch_rotated,
};

use image::{
    ImageRgba8,
    DynamicImage,
    GenericImage,
    ImageBuf,
};

struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.w * self.h
    }
}

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
    fn find_free_area(&self, image: &DynamicImage) -> Option<uint> {
        let mut index = None;
        let mut min_area = None;
        let (image_width, image_height) = image.dimensions();

        for i in range(0, self.free_areas.len()) {
            let ref rect = self.free_areas[i];
            let area = rect.area();
            if image_width <= rect.w && image_height <= rect.h ||
               image_height <= rect.w && image_width <= rect.h {
                if min_area.is_none() || area < min_area.unwrap() {
                    index = Some(i);
                    min_area = Some(area);
                }
            }
        }

        index
    }

    // Shorter Axis Split
    fn split(&mut self, index: uint, rotated: bool, image: &DynamicImage) {
        let (mut image_width, mut image_height) = image.dimensions();
        let rect = self.free_areas.remove(index).unwrap();

        if rotated {
            let tmp = image_width;
            image_width = image_height;
            image_height = tmp;
        }

        // Split horizontally
        if rect.w < rect.h {
            self.free_areas.push(Rect {
                x: rect.x + image_width,
                y: rect.y,
                w: rect.w - image_width,
                h: image_height,
            });

            self.free_areas.push(Rect {
                x: rect.x,
                y: rect.y + image_height,
                w: rect.w,
                h: rect.h - image_height,
            });
        }
        // Split vertically
        else {
            self.free_areas.push(Rect {
                x: rect.x,
                y: rect.y + image_height,
                w: image_width,
                h: rect.h - image_height,
            });

            self.free_areas.push(Rect {
                x: rect.x + image_width,
                y: rect.y,
                w: rect.w - image_width,
                h: rect.h,
            });
        }
    }
}

impl Packer for GuillotinePacker {
    fn pack(&mut self, image: &DynamicImage) {
        let index = self.find_free_area(image);
        if index.is_some() {
            let i = index.unwrap();
            let rect = self.free_areas[i];
            let (image_width, image_height) = image.dimensions();
            let mut rotated = false;

            if image_width <= rect.w && image_height <= rect.h {
                patch(&mut self.buf, rect.x, rect.y, image);
            } else {
                patch_rotated(&mut self.buf, rect.x, rect.y, image);
                rotated = true;
            }

            self.split(i, rotated, image);
        }
    }

    fn image(&self) -> &DynamicImage {
        &self.buf
    }
}

