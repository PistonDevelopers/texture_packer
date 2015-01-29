use texture::Texture;

pub use self::image_exporter::ImageExporter;

mod image_exporter;

pub type ExportResult<T> = Result<T, String>;

pub trait Exporter<T: Texture> {
    type Output;

    fn export(texture: &T) -> ExportResult<Self::Output>;
}
