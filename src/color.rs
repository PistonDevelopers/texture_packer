
use image::{
    Rgba,
    Rgb,
    Luma,
};

#[deriving(Copy)]
pub enum ColorType {
    Grey,
    RGB,
    RGBA,
}

#[deriving(Copy)]
pub enum Color {
    RGBA8(Rgba<u8>),
    RGB8(Rgb<u8>),
    Grey8(Luma<u8>),
}

