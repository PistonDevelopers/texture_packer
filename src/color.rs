
use image::{
    Rgba,
    Rgb,
    Luma,
};

pub enum ColorType {
    Grey,
    RGB,
    RGBA,
}

pub enum Color {
    RGBA8(Rgba<u8>),
    RGB8(Rgb<u8>),
    Grey8(Luma<u8>),
}

