
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

    fn find_free_area(&self, image: &DynamicImage) -> Option<uint> {
        let (image_width, image_height) = image.dimensions();
        let mut min_x = None;
        let mut min_y = None;
        let mut index = None;

        for i in range(0, self.free_areas.len()) {
            let ref free_area = self.free_areas[i];

            if free_area.w >= image_width && free_area.h >= image_height ||
               free_area.h >= image_width && free_area.w >= image_height {
                if min_y.is_none() || free_area.top() < min_y.unwrap() ||
                   (free_area.top() == min_y.unwrap() && free_area.left() < min_x.unwrap()) {
                    index = Some(i);
                    min_x = Some(free_area.left());
                    min_y = Some(free_area.top());
                }
            }
        }

        index
    }

    fn split(&mut self, index: uint, w: u32, h: u32) {
        let rect = self.free_areas.remove(index).unwrap();
        self.free_areas.push(Rect::new_with_points(rect.left() + w, rect.top(), rect.right(), rect.bottom()));
        self.free_areas.push(Rect::new_with_points(rect.left(), rect.top() + h, rect.right(), rect.bottom()));
    }

    fn divide(&mut self, rect: &Rect) {
        let mut new_free_areas = Vec::new();
        for free_area in self.free_areas.iter() {
            new_free_areas = new_free_areas.append(free_area.crop(rect).as_slice());
        }
        self.free_areas = new_free_areas;
    }

    fn cleanup(&mut self) {
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
    fn pack(&mut self, image: &DynamicImage) {
        let index = self.find_free_area(image);
        if index.is_some() {
            let i = index.unwrap();
            let free_area = self.free_areas[i];
            let (image_width, image_height) = image.dimensions();
            let mut rect = Rect::new(free_area.x, free_area.y, image_width, image_height);

            if image_width <= free_area.w && image_height <= free_area.h {
                patch(&mut self.buf, free_area.x, free_area.y, image);
            } else {
                patch_rotated(&mut self.buf, free_area.x, free_area.y, image);
                rect.w = image_height;
                rect.h = image_width;
            }

            self.split(i, rect.w, rect.h);
            self.divide(&rect);
            self.cleanup();
        }
    }

    fn image(&self) -> &DynamicImage {
        &self.buf
    }
}
