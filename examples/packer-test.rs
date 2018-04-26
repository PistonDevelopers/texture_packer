extern crate image;
extern crate texture_packer;

use std::path::Path;
use std::fs::File;

use texture_packer::texture::Texture;
use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::importer::ImageImporter;
use texture_packer::exporter::ImageExporter;

fn main() {
    //
    // Perform texture packing
    //
    let config = TexturePackerConfig {
        max_width: 400,
        max_height: 400,
        allow_rotation: false,
        texture_outlines: true,
        border_padding: 2,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    for i in 1..11 {
        let name = format!("{}.png", i);
        let path = format!("examples/assets/{}", name);
        let path = Path::new(&path);
        let texture = ImageImporter::import_from_file(&path).unwrap();

        packer.pack_own(name, texture);
    }


    //
    // Print the information
    //
    println!("Dimensions : {}x{}", packer.width(), packer.height());
    println!("");
    for (name, frame) in packer.get_frames() {
        println!("  {:7} : {:?}", name, frame.frame);
    }


    //
    // Save the result
    //
    let exporter = ImageExporter::export(&packer).unwrap();
    let mut file = File::create("examples/output/skyline-packer-output.png").unwrap();
    exporter.write_to(&mut file, image::PNG).unwrap();
}
