//! Defines a trait for exporting [Texture]s to arbitrary data.
use image::Rgba;
pub use self::image_exporter::ImageExporter;
use crate::texture::Texture;

mod image_exporter;

/// Result of exporting a texture.
pub type ExportResult<T> = Result<T, String>;

/// Background color settings for sections of the image without regions.
pub struct BackgroundColorSettings {
    /// Color to use on exported image for sections that have no image region pixels.
    pub color: Rgba<u8>,
    /// Threshold value for overriding image region's own pixel color value with background [color] value if
    /// the image region pixel color value's alpha is below or equal to the [region_transparency_threshold].
    ///
    /// If `None`, the region's own pixel value will be written without any modifications.
    pub region_transparency_threshold: Option<u8>,
    /// If set to `true`, [region_transparency_threshold] is set to `true` and the threshold test uses region's own color,
    /// replace the region color alpha with 255 making it fully opaque. This is useful if you want to
    /// get rid of leaky transparent border pixels inside a region.
    pub discard_own_alpha_on_threshold_test: bool,
}

/// Converter for a [Texture] to type `Output`.
pub trait Exporter<T: Texture> {
    /// Output type to use.
    type Output;

    /// Export a texture object to an `Output`.
    fn export(texture: &T, background_color: Option<BackgroundColorSettings>) -> ExportResult<Self::Output>;
}
