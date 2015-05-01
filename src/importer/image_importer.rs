use std::path::Path;

use importer::{
    Importer,
    ImportResult,
};

use image;
use image::{
    DynamicImage,
};

#[derive(Copy, Clone)]
pub struct ImageImporter;

impl ImageImporter {
    pub fn import_from_file(file: &Path) -> ImportResult<DynamicImage> {
        <Self as Importer<&Path>>::import(file)
    }

    pub fn import_from_memory(buf: &[u8]) -> ImportResult<DynamicImage> {
        <Self as Importer<&[u8]>>::import(buf)
    }
}

impl<'a> Importer<&'a Path> for ImageImporter {
    type Texture = DynamicImage;

    fn import(input: &Path) -> ImportResult<DynamicImage> {
        match image::open(input) {
            Ok(image) => Ok(image),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl<'a> Importer<&'a [u8]> for ImageImporter {
    type Texture = DynamicImage;

    fn import(input: &[u8]) -> ImportResult<DynamicImage> {

        match image::load_from_memory(input) {
            Ok(image) => Ok(image),
            Err(e) => Err(format!("{}", e)),
        }
    }
}
