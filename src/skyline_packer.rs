
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

struct Skyline {
    pub x: u32,
    pub y: u32,
    pub w: u32,
}

pub struct SkylinePacker {
    buf: DynamicImage,
    skylines: Vec<Skyline>,
}

impl SkylinePacker {
    pub fn new(width: u32, height: u32) -> SkylinePacker {
        let mut skylines = Vec::new();
        skylines.push(Skyline {
            x: 0,
            y: 0,
            w: width,
        });

        SkylinePacker {
            buf: ImageRgba8(ImageBuf::new(width, height)),
            skylines: skylines,
        }
    }

    fn find_skyline(&self, w: u32, h: u32) -> Option<uint> {
        let (_, buf_height) = self.buf.dimensions();

        for index in range(0, self.skylines.len()) {
            let ref skyline = self.skylines[index];
            if w <= skyline.w && skyline.y + h <= buf_height ||
               h <= skyline.w && skyline.y + w <= buf_height {
                return Some(index);
            }
        }

        None
    }

    fn split(&mut self, index: uint, w: u32, h: u32) {
        let skyline = self.skylines.remove(index).unwrap();
        self.skylines.push(Skyline {
            x: skyline.x + w,
            y: skyline.y,
            w: skyline.w - w,
        });
        self.skylines.push(Skyline {
            x: skyline.x,
            y: skyline.y + h,
            w: w,
        });
    }

    fn merge(&mut self) {
        self.skylines.sort_by(|a, b| {
            if a.y < b.y {
                Less
            } else if a.y == b.y {
                if a.x < b.x {
                    Less
                } else if a.x == b.x {
                    Equal
                } else {
                    Greater
                }
            } else {
                Greater
            }
        });

        let mut new_skylines = Vec::new();
        let mut removed = Vec::new();

        for i in range(0, self.skylines.len()) {
            if removed.contains(&i) {
                continue
            }

            let mut new_skyline = self.skylines[i];

            for j in range (i + 1, self.skylines.len()) {
                if new_skyline.y == self.skylines[j].y &&
                   new_skyline.x + new_skyline.w == self.skylines[j].x {
                    new_skyline.w += self.skylines[j].w;
                    removed.push(j);
                }
            }
            new_skylines.push(new_skyline);
        }
        self.skylines = new_skylines;
    }
}

impl Packer for SkylinePacker {
    fn pack(&mut self, image: &DynamicImage) {
        let (image_width, image_height) = image.dimensions();
        let index = self.find_skyline(image_width, image_height);
        if index.is_some() {
            let i = index.unwrap();
            let skyline = self.skylines[i];
            let (_, buf_height) = self.buf.dimensions();
            let mut patched_width = image_width;
            let mut patched_height = image_height;

            if image_width <= skyline.w && skyline.y + image_height <= buf_height {
                patch(&mut self.buf, skyline.x, skyline.y, image);
            } else {
                patch_rotated(&mut self.buf, skyline.x, skyline.y, image);
                patched_width = image_height;
                patched_height = image_width;
            }

            self.split(i, patched_width, patched_height);
            self.merge();
        }
    }

    fn image(&self) -> &DynamicImage {
        &self.buf
    }
}
