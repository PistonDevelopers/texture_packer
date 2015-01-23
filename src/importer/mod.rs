use texture::Texture;

pub trait Importer {
    type Texture: Texture;

    fn import_from_file(filename: &str) -> Self::Texture;
}
