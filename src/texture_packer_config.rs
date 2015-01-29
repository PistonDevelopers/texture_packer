#[derive(Copy)]
pub struct TexturePackerConfig {
    pub algorithm: TexturePackerAlrogithm,

    //
    // layout configuration
    //
    pub max_width: u32,
    pub max_height: u32,
    pub allow_rotation: bool,

    //
    // texture configuration
    //
    pub border_padding: u32,
    pub shape_padding: u32,
}

impl TexturePackerConfig {
    pub fn default() -> TexturePackerConfig {
        TexturePackerConfig {
            algorithm: TexturePackerAlrogithm::Skyline,

            max_width: 1024,
            max_height: 1024,
            allow_rotation: true,

            border_padding: 0,
            shape_padding: 2,
        }
    }
}

#[derive(Copy)]
pub enum TexturePackerAlrogithm {
    Skyline,
}
