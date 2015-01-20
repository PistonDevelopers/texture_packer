
use image::{
    Rgba,
    Rgb,
    Luma,
};

#[derive(Copy, Clone)]
pub enum ColorType {
    Grey,
    RGB,
    RGBA,
}

#[derive(Copy, Clone)]
pub enum Color {
    RGBA8(Rgba<u8>),
    RGB8(Rgb<u8>),
    Grey8(Luma<u8>),
}

