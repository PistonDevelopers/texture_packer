#![feature(collections, core, path, io)]

extern crate image;
extern crate texture_packer;

use std::old_io::File;

use texture_packer::texture::Texture;
use texture_packer::{
    TexturePacker,
    TexturePackerConfig,
};
use texture_packer::importer::{
    ImageImporter,
};
use texture_packer::exporter::{
    ImageExporter,
};

static MAX_IMAGE_WIDTH: u32 = 400;
static MAX_IMAGE_HEIGHT: u32 = 400;

fn main() {
    let mut config = TexturePackerConfig::default();
    config.max_width = MAX_IMAGE_WIDTH;
    config.max_height = MAX_IMAGE_HEIGHT;
    config.allow_rotation = false;
    config.texture_outlines = true;
    config.border_padding = 2;

    let mut texture_packer = TexturePacker::new_skyline(config);

    for i in 1..11 {
        let mut path = "./assets/".to_string();
        let filename = format!("{}.png", i);
        path.push_str(filename.as_slice());
        let texture = ImageImporter::import_from_file(&Path::new(path)).unwrap();

        texture_packer.pack_own(filename.clone(), texture);
    }

    let image = ImageExporter::export(&texture_packer).unwrap();
    let output_filename = "skyline-packer-output.png";
    let fout = File::create(&Path::new(output_filename)).unwrap();
    println!("{} x {}", texture_packer.width(), texture_packer.height());
    let _ = image.save(fout, image::PNG);
}

