
extern crate image;
extern crate texture_packer;

use std::io::File;

use texture_packer::{
    ColorType,
    ImgBuffer,
    Packer,
    ShelfPacker,
    GuillotinePacker,
    MaxrectPacker,
    SkylinePacker,
};

static OUTPUT_IMAGE_WIDTH: u32 = 400;
static OUTPUT_IMAGE_HEIGHT: u32 = 400;

fn pack(packer: &mut Packer<ImgBuffer>, output_filename: &str) {
    for i in range(1u32, 11) {
        let mut filename = String::from_str("./assets/");
        filename.push_str(format!("{}.png", i).as_slice());
        let image = ImgBuffer::open(&Path::new(filename)).unwrap();

        packer.set_margin(1);
        packer.pack(&image);
    }

    let fout = File::create(&Path::new(output_filename)).unwrap();
    let _ = packer.buf().image().save(fout, image::PNG);
}

fn main() {
    pack(&mut ShelfPacker::new(ImgBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, ColorType::RGBA)), "shelf-packer-output.png");
    pack(&mut GuillotinePacker::new(ImgBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, ColorType::RGBA)), "guillotine-packer-output.png");
    pack(&mut MaxrectPacker::new(ImgBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, ColorType::RGBA)), "maxrect-packer-output.png");
    pack(&mut SkylinePacker::new(ImgBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, ColorType::RGBA)), "skyline-packer-output.png");
}

