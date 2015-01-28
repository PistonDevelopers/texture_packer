#![allow(unstable)]

extern crate image;
extern crate texture_packer;

use std::io::File;

use texture_packer::{
    Packer,
    ShelfPacker,
    GuillotinePacker,
    MaxrectPacker,
    SkylinePacker,
};

use image::{
    ImageBuffer,
    DynamicImage
};

static OUTPUT_IMAGE_WIDTH: u32 = 400;
static OUTPUT_IMAGE_HEIGHT: u32 = 400;

fn pack<P: Packer<Buffer=DynamicImage>>(output_filename: &str) {
    let buf = DynamicImage::ImageRgba8(ImageBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT));
    let mut packer: P = Packer::new(buf);

    for i in range(1u32, 11) {
        let mut filename = "./assets/".to_string();
        filename.push_str(format!("{}.png", i).as_slice());
        let image = image::open(&Path::new(filename)).unwrap();

        packer.set_margin(1);
        packer.pack(&image);
    }

    let fout = File::create(&Path::new(output_filename)).unwrap();
    let _ = packer.buf().save(fout, image::PNG);
}

fn main() {
    pack::<ShelfPacker<DynamicImage>>("shelf-packer-output.png");
    pack::<MaxrectPacker<DynamicImage>>("maxrect-packer-output.png");
    pack::<SkylinePacker<DynamicImage>>("skyline-packer-output.png");
    pack::<GuillotinePacker<DynamicImage>>("guillotine-packer-output.png");
}

