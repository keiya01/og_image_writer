use super::font::FontArc;
use super::layout::TextArea;
use super::style::{Margin, Position, Style};
use image::{ImageBuffer, Rgba};
use std::ops::Range;

#[derive(Debug)]
pub(super) enum Element {
    Img(Option<Img>),
    Text(Option<Text>),
}

impl Element {
    pub(super) fn is_absolute(&self) -> bool {
        match self {
            Element::Img(Some(img)) => matches!(img.style.position, Position::Absolute),
            Element::Text(Some(text)) => matches!(text.style.position, Position::Absolute),
            _ => false,
        }
    }

    pub(super) fn margin(&self) -> Margin {
        match self {
            Element::Img(Some(img)) => img.style.margin,
            Element::Text(Some(text)) => text.style.margin,
            _ => Margin::default(),
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct Rect {
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
pub(super) struct Img {
    pub(super) buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub(super) rect: Rect,
    pub(super) style: Style,
}

impl Img {
    pub fn new(buf: ImageBuffer<Rgba<u8>, Vec<u8>>, rect: Rect, style: Style) -> Self {
        Img { buf, rect, style }
    }
}

#[derive(Debug)]
pub(super) struct Fragment {
    pub(super) range: Range<usize>,
    pub(super) rect: Rect,
}

impl Fragment {
    pub fn new(range: Range<usize>, rect: Rect) -> Self {
        Fragment { range, rect }
    }
}

#[derive(Debug, Default)]
pub(super) struct LineMetrics {
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
pub(super) struct Text {
    pub(super) text: String,
    pub(super) metrics: LineMetrics,
    pub(super) fragments: Vec<Fragment>,
    pub(super) style: Style,
    // TODO: optimize static lifetime
    pub(super) font: Option<FontArc>,
    pub(super) textarea: TextArea,
}

impl Text {
    pub fn new(
        text: String,
        fragments: Vec<Fragment>,
        metrics: LineMetrics,
        style: Style,
        font: Option<FontArc>,
        textarea: TextArea,
    ) -> Self {
        Text {
            text,
            fragments,
            metrics,
            style,
            font,
            textarea,
        }
    }
}
