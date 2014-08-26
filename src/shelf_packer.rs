
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

pub struct ShelfPacker {
    buf: DynamicImage,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    opening_shelf_max_y: u32,
}

impl ShelfPacker {
    pub fn new(width: u32, height: u32) -> ShelfPacker {
        ShelfPacker {
            buf: ImageRgba8(ImageBuf::new(width, height)),
            width: width,
            height: height,
            x: 0,
            y: 0,
            opening_shelf_max_y: 0,
        }
    }

    pub fn image(&self) -> &DynamicImage {
        &self.buf
    }
}

impl Packer for ShelfPacker {
    fn pack(&mut self, image: &DynamicImage) {
        let (image_width, image_height) = image.dimensions();

        let mut patch_fn = patch;
        let mut patched_width = image_width;
        let mut patched_height = image_height;

        // If the rectangle is the first rectangle on a new open shelf,
        // store it sideways. This is to minimize the height of the new shelf.
        if self.x == 0 {
            if image_height > image_width && image_height <= self.width {
                patch_fn = patch_rotated;
                patched_width = image_height;
                patched_height = image_width;
            }
        }

        // If the rectangle fits upright then store it so.
        // This aims to minimize the wasted surface area between the
        // rectangle top side and the shelf ceiling.
        //
        // Otherwise store the rectangle sideways if possible.
        else {
            if image_width > image_height && self.x + image_height <= self.width {
                patch_fn = patch_rotated;
                patched_width = image_height;
                patched_height = image_width;
            } else if self.x + image_width > self.width {
                // Open a new shelf
                self.x = 0;
                self.y += self.opening_shelf_max_y;
                self.opening_shelf_max_y = 0;
            }
        }

        if self.x + patched_width <= self.width && self.y + patched_height <= self.height {
            patch_fn(&mut self.buf, self.x, self.y, image);
            self.x += patched_width;
            if self.opening_shelf_max_y < patched_height {
                self.opening_shelf_max_y = patched_height;
            }
        }
    }
}

