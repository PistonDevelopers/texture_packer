
extern crate image;

use std::io::File;
use packer::Packer;

mod packer;
mod shelf_packer;

fn main() {
    let mut packer = shelf_packer::ShelfPacker::new(400, 400);

    for i in range(1u32, 10) {
        let mut filename = String::from_str("./bin/");
        filename.push_str(format!("{}.png", i).as_slice());
        let image = image::open(&Path::new(filename)).unwrap();

        packer.pack(&image);
    }

    let fout = File::create(&Path::new("output.png")).unwrap();
    let _ = packer.image().save(fout, image::PNG);
}

