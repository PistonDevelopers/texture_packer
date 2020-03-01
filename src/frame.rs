use crate::rect::Rect;

/// Boundaries and properties of a packed texture.
#[derive(Clone, Debug)]
pub struct Frame {
    /// Key used to uniquely identify this frame.
    pub key: String,
    /// Rectangle describing the texture coordinates and size.
    pub frame: Rect,
    /// True if the texture was rotated during packing.
    pub rotated: bool,
    /// True if the texture was trimmed during packing.
    pub trimmed: bool,

    // (x, y) is the trimmed frame position at original image
    // (w, h) is original image size
    //
    //            w
    //     +--------------+
    //     | (x, y)       |
    //     |  ^           |
    //     |  |           |
    //     |  *********   |
    //     |  *       *   |  h
    //     |  *       *   |
    //     |  *********   |
    //     |              |
    //     +--------------+
    /// Source texture size before any trimming.
    pub source: Rect,
}
