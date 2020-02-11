extern crate image;
extern crate texture_packer;

use std::{fs::File, path::Path};
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, texture::Texture, MultiTexturePacker,
    TexturePacker, TexturePackerConfig,
};

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

    // single atlas
    {
        let mut packer = TexturePacker::new_skyline(config);

        for i in 1..11 {
            let name = format!("{}.png", i);
            let path = format!("examples/assets/{}", name);
            let path = Path::new(&path);
            let texture = ImageImporter::import_from_file(&path).unwrap();

            packer.pack_own(name, texture).unwrap();
        }

        //
        // Print the information
        //
        println!("Dimensions : {}x{}", packer.width(), packer.height());
        for (name, frame) in packer.get_frames() {
            println!("  {:7} : {:?}", name, frame.frame);
        }

        //
        // Save the result
        //
        let exporter = ImageExporter::export(&packer).unwrap();
        let mut file = File::create("examples/output/skyline-packer-output.png").unwrap();
        exporter.write_to(&mut file, image::ImageFormat::Png).unwrap();
    }

    // multiple atlases
    {
        let mut packer = MultiTexturePacker::new_skyline(config);

        for i in 1..11 {
            let name = format!("{}.png", i);
            let path = format!("examples/assets/{}", name);
            let path = Path::new(&path);
            let texture = ImageImporter::import_from_file(&path).unwrap();

            packer.pack_own(format!("A{}", i), texture.clone()).unwrap();
            packer.pack_own(format!("B{}", i), texture).unwrap();
        }

        for (i, page) in packer.get_pages().into_iter().enumerate() {
            //
            // Print the information
            //
            println!("#{} | Dimensions : {}x{}", i, page.width(), page.height());
            for (name, frame) in page.get_frames() {
                println!("#{} |   {:7} : {:?}", i, name, frame.frame);
            }

            //
            // Save the result
            //
            let exporter = ImageExporter::export(page).unwrap();
            let mut file = File::create(&format!(
                "examples/output/skyline-multi-packer-output-{}.png",
                i
            ))
            .unwrap();
            exporter.write_to(&mut file, image::ImageFormat::Png).unwrap();
        }
    }
}
