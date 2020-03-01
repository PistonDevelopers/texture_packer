use crate::{
    packer::{Packer, SkylinePacker},
    texture::{Pixel, Texture},
    texture_packer::{PackResult, TexturePacker},
    texture_packer_config::TexturePackerConfig,
};

/// Texture packer supporting multiple atlases.
///
/// Will create a new page for textures that do not fit. Textures packed after a new page is added
/// will still attempt to check each page for available space.
pub struct MultiTexturePacker<'a, T: 'a + Clone, P> {
    config: TexturePackerConfig,
    pages: Vec<TexturePacker<'a, T, P>>,
}

impl<'a, Pix: Pixel, T: Clone + Texture<Pixel = Pix>, P: Packer<Pixel = Pix>>
    MultiTexturePacker<'a, T, P>
{
    /// Get an array of all underlying single-atlas texture packers.
    pub fn get_pages(&self) -> &[TexturePacker<'a, T, P>] {
        &self.pages
    }
}

impl<'a, Pix: Pixel, T: 'a + Clone + Texture<Pixel = Pix>>
    MultiTexturePacker<'a, T, SkylinePacker<Pix>>
{
    /// Create a new packer using the skyline packing algorithm.
    pub fn new_skyline(config: TexturePackerConfig) -> Self {
        Self {
            config,
            pages: vec![],
        }
    }
}

impl<'a, Pix: Pixel, T: Clone + Texture<Pixel = Pix>>
    MultiTexturePacker<'a, T, SkylinePacker<Pix>>
{
    /// Pack the `texture` into this packer, taking a reference of the texture object.
    pub fn pack_ref(&mut self, key: String, texture: &'a T) -> PackResult<()> {
        for packer in &mut self.pages {
            if packer.can_pack(texture) {
                return packer.pack_ref(key, texture);
            }
        }
        let mut packer = TexturePacker::new_skyline(self.config.clone());
        packer.pack_ref(key, texture)?;
        self.pages.push(packer);
        Ok(())
    }

    /// Pack the `texture` into this packer, taking ownership of the texture object.
    pub fn pack_own(&mut self, key: String, texture: T) -> PackResult<()> {
        for packer in &mut self.pages {
            if packer.can_pack(&texture) {
                return packer.pack_own(key, texture);
            }
        }
        let mut packer = TexturePacker::new_skyline(self.config.clone());
        packer.pack_own(key, texture)?;
        self.pages.push(packer);
        Ok(())
    }
}
