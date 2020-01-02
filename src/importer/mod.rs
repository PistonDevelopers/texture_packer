pub use self::image_importer::ImageImporter;
use crate::texture::Texture;

mod image_importer;

pub type ImportResult<T> = Result<T, String>;

pub trait Importer<I> {
    type Texture: Texture;

    fn import(input: I) -> ImportResult<Self::Texture>;
}
