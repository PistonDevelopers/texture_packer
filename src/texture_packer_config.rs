#[derive(Copy, Clone)]
pub struct TexturePackerConfig {
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
    pub texture_padding: u32,

    pub trim: bool,

    pub texture_outlines: bool,
}

impl TexturePackerConfig {
    pub fn default() -> TexturePackerConfig {
        TexturePackerConfig {
            max_width: 1024,
            max_height: 1024,
            allow_rotation: true,

            border_padding: 0,
            texture_padding: 2,

            trim: true,

            texture_outlines: false,
        }
    }
}
