extern crate image;
extern crate texture_packer;

use std::path::Path;
use std::fs::File;

use texture_packer::texture::Texture;
use texture_packer::{ TexturePacker, TexturePackerConfig };
use texture_packer::importer::ImageImporter;
use texture_packer::exporter::ImageExporter;

const MAX_IMAGE_WIDTH: u32 = 400;
const MAX_IMAGE_HEIGHT: u32 = 400;

fn main() {
    let mut config = TexturePackerConfig::default();
    config.max_width = MAX_IMAGE_WIDTH;
    config.max_height = MAX_IMAGE_HEIGHT;
    config.allow_rotation = false;
    config.texture_outlines = true;
    config.border_padding = 2;

    let ref mut texture_packer = TexturePacker::new_skyline(config);

    for i in 1 .. 11 {
        let file = format!("{}.png", i);
        let ref path = ["./examples/assets/", &file[..]].concat();
        let ref path = Path::new(path);
        let texture = ImageImporter::import_from_file(path).unwrap();

        texture_packer.pack_own(file, texture);
    }

    let image = ImageExporter::export(texture_packer).unwrap();
    let path = "./examples/output/skyline-packer-output.png";
    let ref path = Path::new(path);
    let ref mut file = File::create(path).unwrap();

    println!("{} x {}", texture_packer.width(), texture_packer.height());
    image.save(file, image::PNG).unwrap();
}
