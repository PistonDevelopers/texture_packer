use std::default::Default;

#[derive(Copy, Clone)]
pub struct TexturePackerConfig {
    //
    // layout configuration
    //
    /// Max width of the packed image. Default value is `1024`.
    pub max_width: u32,
    /// Max height of the packed image. Default value is `1024`.
    pub max_height: u32,
    /// True to allow rotation of the input images. Default value is `true`.
    pub allow_rotation: bool,

    //
    // texture configuration
    //
    /// Size of the padding on the outer edge of the packed image in pixel. Default value is `0`.
    pub border_padding: u32,
    /// Size of the padding between frames in pixel. Default value is `2`
    pub texture_padding: u32,

    /// True to trim the empty pixels of the input images. Default value is `true`.
    pub trim: bool,

    /// True to draw the red line on the edge of the each frames. Useful for debugging. Default
    /// value is `false`.
    pub texture_outlines: bool,
}

impl Default for TexturePackerConfig {
    fn default() -> TexturePackerConfig {
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
