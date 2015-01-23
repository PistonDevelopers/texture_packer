use Texture;

pub trait Exporter<T: Texture> {
    fn export(texture: &T);
}
