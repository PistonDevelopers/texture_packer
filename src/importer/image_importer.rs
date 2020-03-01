use crate::importer::{ImportResult, Importer};
use image::{self, DynamicImage};
use std::path::Path;

/// Importer type for images.
#[derive(Copy, Clone)]
pub struct ImageImporter;

impl ImageImporter {
    /// Import an image from a path.
    pub fn import_from_file(file: &Path) -> ImportResult<DynamicImage> {
        <Self as Importer<&Path>>::import(file)
    }

    /// Import an image from memory.
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
