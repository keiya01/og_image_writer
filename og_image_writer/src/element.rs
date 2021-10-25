use super::layout::TextArea;
use super::style::{Margin, Position, Style};
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

    pub(super) fn margin(&self) -> Margin {
        match self {
            Element::Img(Some(img)) => img.style.margin.clone(),
            Element::Text(Some(text)) => text.style.margin.clone(),
            _ => Margin::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Rect {
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) width: u32,
    pub(super) height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub struct Img<'a> {
    pub(super) buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub(super) rect: Rect,
    pub(super) style: Style<'a>,
}

impl<'a> Img<'a> {
    pub fn new(buf: ImageBuffer<Rgba<u8>, Vec<u8>>, rect: Rect, style: Style<'a>) -> Self {
        Img { buf, rect, style }
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

#[derive(Debug, Default)]
pub struct LineMetrics {
    pub total_height: u32,
    pub max_line_height: f32,
    pub max_line_width: f32,
}

impl LineMetrics {
    pub fn new(total_height: u32, max_line_height: f32, max_line_width: f32) -> Self {
        LineMetrics {
            total_height,
            max_line_height,
            max_line_width,
        }
    }
}

#[derive(Debug)]
pub struct Text<'a> {
    pub(super) text: String,
    pub(super) metrics: LineMetrics,
    pub(super) lines: Vec<Line>,
    pub(super) style: Style<'a>,
    pub(super) font: Font<'a>,
    pub(super) textarea: TextArea<'a>,
}

impl<'a> Text<'a> {
    pub fn new(
        text: String,
        lines: Vec<Line>,
        metrics: LineMetrics,
        style: Style<'a>,
        font: Font<'a>,
        textarea: TextArea<'a>,
    ) -> Self {
        Text {
            text,
            lines,
            metrics,
            style,
            font,
            textarea,
        }
    }
}
