use super::style::{Position, Style};
use image::{ImageBuffer, Rgba};
use rusttype::Font;
use std::ops::Range;

#[derive(Debug)]
pub(super) enum Element<'a> {
    Img(Option<Img<'a>>),
    Text(Option<Text<'a>>),
}

impl<'a> Element<'a> {
    pub(super) fn is_absolute(&self) -> bool {
        match self {
            Element::Img(Some(img)) => matches!(img.style.position, Position::Absolute),
            Element::Text(Some(text)) => matches!(text.style.position, Position::Absolute),
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct Rect {
    pub(super) x: u32,
    pub(super) y: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32) -> Self {
        Rect { x, y }
    }
}

#[derive(Debug)]
pub struct Img<'a> {
    pub(super) buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rect: Rect,
    pub(super) style: Style<'a>,
}

impl<'a> Img<'a> {
    pub fn new(
        buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
        width: u32,
        height: u32,
        rect: Rect,
        style: Style<'a>,
    ) -> Self {
        Img {
            buf,
            width,
            height,
            rect,
            style,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub(super) range: Range<usize>,
    pub(super) rect: Rect,
}

impl Line {
    pub fn new(range: Range<usize>, rect: Rect) -> Self {
        Line { range, rect }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub(super) text: String,
    pub(super) total_height: u32,
    pub(super) lines: Vec<Line>,
    pub(super) style: Style<'a>,
    pub(super) font: Font<'a>,
    pub(super) max_line_height: f32,
}

impl<'a> Text<'a> {
    pub fn new(
        text: String,
        lines: Vec<Line>,
        total_height: u32,
        style: Style<'a>,
        font: Font<'a>,
        max_line_height: f32,
    ) -> Self {
        Text {
            text,
            lines,
            total_height,
            style,
            font,
            max_line_height,
        }
    }
}
