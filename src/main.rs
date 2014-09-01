
extern crate texture_packer;

use texture_packer::{
    RGBA,
    ImageBuffer,
    Packer,
    ShelfPacker,
    GuillotinePacker,
    MaxrectPacker,
    SkylinePacker,
};

static OUTPUT_IMAGE_WIDTH: u32 = 400;
static OUTPUT_IMAGE_HEIGHT: u32 = 400;

fn pack(packer: &mut Packer, output_filename: &str) {
    for i in range(1u32, 11) {
        let mut filename = String::from_str("./bin/");
        filename.push_str(format!("{}.png", i).as_slice());
        let image = ImageBuffer::open(&Path::new(filename)).unwrap();

        packer.set_margin(1);
        packer.pack(&image);
    }

    let _ = packer.buf().save(&Path::new(output_filename));
}

fn main() {
    pack(&mut ShelfPacker::new(&mut ImageBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, RGBA)), "shelf-packer-output.png");
    pack(&mut GuillotinePacker::new(&mut ImageBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, RGBA)), "guillotine-packer-output.png");
    pack(&mut MaxrectPacker::new(&mut ImageBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, RGBA)), "maxrect-packer-output.png");
    pack(&mut SkylinePacker::new(&mut ImageBuffer::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT, RGBA)), "skyline-packer-output.png");
}

