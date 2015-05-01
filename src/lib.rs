extern crate image;

use std::ops::Deref;
pub use rect::Rect;
pub use frame::Frame;
pub use texture_packer::TexturePacker;
pub use texture_packer_config::TexturePackerConfig;

pub mod texture;
pub mod importer;
pub mod exporter;

mod rect;
mod frame;
mod texture_packer;
mod texture_packer_config;
mod packer;

enum Cow<'a, T: 'a> {
    Borrowed(&'a T),
    Owned(T)
}

impl <'a, T: 'a> Deref for Cow<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            &Cow::Borrowed(t) => t,
            &Cow::Owned(ref t) => t
        }
    }
}
