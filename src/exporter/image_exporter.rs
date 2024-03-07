use crate::{
    exporter::{ExportResult, Exporter},
    texture::Texture,
};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::marker::PhantomData;
use crate::exporter::BackgroundColorSettings;

/// Exporter type for images.
#[derive(Copy, Clone)]
pub struct ImageExporter<T>(PhantomData<T>);

impl<T: Texture<Pixel = Rgba<u8>>> ImageExporter<T> {
    /// Export a texture to an image type.
    ///
    /// [background_color]: Background color settings for sections containing no image regions.
    /// See [BackgroundColorSettings] for more information.
    pub fn export(texture: &T, background_color: Option<BackgroundColorSettings>) -> ExportResult<DynamicImage> {
        <Self as Exporter<T>>::export(texture, background_color)
    }
}

impl<T: Texture<Pixel = Rgba<u8>>> Exporter<T> for ImageExporter<T> {
    type Output = DynamicImage;

    fn export(texture: &T, background_color: Option<BackgroundColorSettings>) -> ExportResult<DynamicImage> {
        let width = texture.width();
        let height = texture.height();

        if width == 0 || height == 0 {
            return Err("Width or height of this texture is zero".to_string());
        }

        let mut pixels = Vec::with_capacity((width * height * 4) as usize);

        match background_color {
            None => {
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
            }
            Some(bg) => {
                let bg_r = bg.color.0[0];
                let bg_g = bg.color.0[1];
                let bg_b = bg.color.0[2];
                let bg_a = bg.color.0[3];
                for row in 0..height {
                    for col in 0..width {
                        if let Some(pixel) = texture.get(col, row) {
                            let region_r = pixel[0];
                            let region_g = pixel[1];
                            let region_b = pixel[2];
                            let region_a = pixel[3];
                            if let Some(rthresh) = bg.region_transparency_threshold {
                                if region_a <= rthresh {
                                    // override region's own color with background color:
                                    pixels.push(bg_r);
                                    pixels.push(bg_g);
                                    pixels.push(bg_b);
                                    pixels.push(bg_a);
                                    continue;
                                }

                                // the threshold test failed, but we don't want the image region to have
                                // any transparent pixels regardless:
                                if bg.discard_own_alpha_on_threshold_test {
                                    pixels.push(region_r);
                                    pixels.push(region_g);
                                    pixels.push(region_b);
                                    pixels.push(255);
                                    continue;
                                }
                            }
                            // apply region's own color:
                            pixels.push(region_r);
                            pixels.push(region_g);
                            pixels.push(region_b);
                            pixels.push(region_a);
                            continue;
                        }
                        // apply background color:
                        pixels.push(bg_r);
                        pixels.push(bg_g);
                        pixels.push(bg_b);
                        pixels.push(bg_a);
                    }
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
