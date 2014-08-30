
extern crate image;

pub use packer::Packer;
pub use shelf_packer::ShelfPacker;
pub use guillotine_packer::GuillotinePacker;
pub use maxrect_packer::MaxrectPacker;
pub use skyline_packer::SkylinePacker;

pub mod packer;
pub mod shelf_packer;
pub mod guillotine_packer;
pub mod maxrect_packer;
pub mod skyline_packer;

mod rect;

