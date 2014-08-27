
extern crate image;

use std::io::File;
use packer::Packer;

mod packer;
mod shelf_packer;
mod guillotine_packer;

static OUTPUT_IMAGE_WIDTH: u32 = 400;
static OUTPUT_IMAGE_HEIGHT: u32 = 400;

fn pack(packer: &mut Packer, output_filename: &str) {
    for i in range(1u32, 10) {
        let mut filename = String::from_str("./bin/");
        filename.push_str(format!("{}.png", i).as_slice());
        let image = image::open(&Path::new(filename)).unwrap();

        packer.pack(&image);
    }

    let fout = File::create(&Path::new(output_filename)).unwrap();
    let _ = packer.image().save(fout, image::PNG);
}

fn main() {
    pack(&mut shelf_packer::ShelfPacker::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT), "shelf-packer-output.png");
    pack(&mut guillotine_packer::GuillotinePacker::new(OUTPUT_IMAGE_WIDTH, OUTPUT_IMAGE_HEIGHT), "guillotine-packer-output.png");
}

