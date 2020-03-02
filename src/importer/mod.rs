//! Defines a trait for importing arbitrary data and converting them to [Texture]s.
pub use self::image_importer::ImageImporter;
use crate::texture::Texture;

mod image_importer;

/// Result of importing data and converting it to a texture.
pub type ImportResult<T> = Result<T, String>;

/// Converter for an input type `I` to a [Texture].
pub trait Importer<I> {
    /// [Texture] type to use.
    type Texture: Texture;

    /// Import `I` yielding a texture object.
    fn import(input: I) -> ImportResult<Self::Texture>;
}
