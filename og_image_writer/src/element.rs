use super::style::Style;
use std::ops::Range;

pub(super) enum Element<'a> {
    Img(Option<Img<'a>>),
    Text(Option<Text<'a>>),
}

#[derive(Default)]
pub struct Rect {
    pub(super) x: f64,
    pub(super) y: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64) -> Self {
        Rect { x, y }
    }
}

pub struct Img<'a> {
    pub(super) data: Vec<u8>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rect: Rect,
    pub(super) style: Style<'a>,
}

impl<'a> Img<'a> {
    pub fn new(data: Vec<u8>, width: u32, height: u32, rect: Rect, style: Style<'a>) -> Self {
        Img {
            data,
            width,
            height,
            rect,
            style,
        }
    }
}

pub struct Line {
    pub(super) range: Range<usize>,
    pub(super) rect: Rect,
}

impl Line {
    pub fn new(range: Range<usize>, rect: Rect) -> Self {
        Line { range, rect }
    }
}

pub struct Text<'a> {
    pub(super) text: String,
    pub(super) total_height: f64,
    pub(super) lines: Vec<Line>,
    pub(super) style: Style<'a>,
}

impl<'a> Text<'a> {
    pub fn new(
        text: String,
        lines: Vec<Line>,
        total_height: f64,
        style: Style<'a>,
    ) -> Self {
        Text {
            text,
            lines,
            total_height,
            style,
        }
    }
}
