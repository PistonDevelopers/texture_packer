//! Defines a trait for exporting [Texture]s to arbitrary data.
pub use self::image_exporter::ImageExporter;
use crate::texture::Texture;

mod image_exporter;

/// Result of exporting a texture.
pub type ExportResult<T> = Result<T, String>;

/// Converter for a [Texture] to type `Output`.
pub trait Exporter<T: Texture> {
    /// Output type to use.
    type Output;

    /// Export a texture object to an `Output`.
    fn export(texture: &T) -> ExportResult<Self::Output>;
}
