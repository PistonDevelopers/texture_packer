//! Library for packing different textures into a texture atlas for efficient GPU rendering.
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

pub use crate::{
    frame::Frame, multi_texture_packer::MultiTexturePacker, rect::Rect,
    texture_packer::TexturePacker, texture_packer_config::TexturePackerConfig,
};

pub mod exporter;
pub mod importer;
pub mod texture;

mod frame;
mod multi_texture_packer;
mod packer;
mod rect;
mod texture_packer;
mod texture_packer_config;
