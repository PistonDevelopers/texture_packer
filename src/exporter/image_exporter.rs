use crate::{
    exporter::{ExportResult, Exporter},
    texture::Texture,
};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::marker::PhantomData;

/// Exporter type for images.
#[derive(Copy, Clone)]
pub struct ImageExporter<T>(PhantomData<T>);

impl<T: Texture<Pixel = Rgba<u8>>> ImageExporter<T> {
    /// Export a texture to an image type.
    ///
    /// [background_color]: If None, the sections of the exported image containing no sub-image regions
    /// will be colored with transparent color black: (0,0,0,0).
    /// If set to Some, these pixels will use the specified color given as `Rgba<u8>` value.
    /// For example, `Some([255, 0, 255, 255].into())` will set the background color to magenta.
    ///
    pub fn export(texture: &T, background_color: Option<Rgba<u8>>) -> ExportResult<DynamicImage> {
        <Self as Exporter<T>>::export(texture, background_color)
    }
}

impl<T: Texture<Pixel = Rgba<u8>>> Exporter<T> for ImageExporter<T> {
    type Output = DynamicImage;

    fn export(texture: &T, background_color: Option<Rgba<u8>>) -> ExportResult<DynamicImage> {
        let width = texture.width();
        let height = texture.height();

        if width == 0 || height == 0 {
            return Err("Width or height of this texture is zero".to_string());
        }

        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        let (bg_r, bg_g, bg_b, bg_a) = match background_color {
            None => {
                (0, 0, 0, 0)
            }
            Some(s) => {
                (s.0[0], s.0[1], s.0[2], s.0[3])
            }
        };
        for row in 0..height {
            for col in 0..width {
                if let Some(pixel) = texture.get(col, row) {
                    pixels.push(pixel[0]);
                    pixels.push(pixel[1]);
                    pixels.push(pixel[2]);
                    pixels.push(pixel[3]);
                } else {
                    pixels.push(bg_r);
                    pixels.push(bg_g);
                    pixels.push(bg_b);
                    pixels.push(bg_a);
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
