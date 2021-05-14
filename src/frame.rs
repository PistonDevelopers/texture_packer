use crate::rect::Rect;

/// Boundaries and properties of a packed texture.
#[derive(Clone, Debug)]
pub struct Frame<K> {
    /// Key used to uniquely identify this frame.
    pub key: K,
    /// Rectangle describing the texture coordinates and size.
    pub frame: Rect,
    /// True if the texture was rotated during packing. 
    /// If it was rotated, it was rotated 90 degrees clockwise.
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
