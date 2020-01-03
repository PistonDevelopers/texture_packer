use crate::{
    exporter::{ExportResult, Exporter},
    image::{DynamicImage, ImageBuffer, Rgba},
    texture::Texture,
};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct ImageExporter<T>(PhantomData<T>);

impl<T: Texture<Pixel = Rgba<u8>>> ImageExporter<T> {
    pub fn export(texture: &T) -> ExportResult<DynamicImage> {
        <Self as Exporter<T>>::export(texture)
    }
}

impl<T: Texture<Pixel = Rgba<u8>>> Exporter<T> for ImageExporter<T> {
    type Output = DynamicImage;

    fn export(texture: &T) -> ExportResult<DynamicImage> {
        let width = texture.width();
        let height = texture.height();

        if width == 0 || height == 0 {
            return Err("Widht or height of this texture is zero".to_string());
        }

        let mut pixels = Vec::new();

        for row in 0..height {
            for col in 0..width {
                if let Some(pixel) = texture.get(col, row) {
                    pixels.push(pixel[0]);
                    pixels.push(pixel[1]);
                    pixels.push(pixel[2]);
                    pixels.push(pixel[3]);
                } else {
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                    pixels.push(0);
                }
            }
        }

        if let Some(image_buffer) =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, pixels)
        {
            Ok(DynamicImage::ImageRgba8(image_buffer))
        } else {
            Err("Can't export texture".to_string())
        }
    }
}
