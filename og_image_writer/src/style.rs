pub use image::{Rgb, Rgba};

pub enum WordBreak {
    Normal,
    BreakAll,
}

#[derive(Default)]
pub struct Margin(pub i32, pub i32, pub i32, pub i32);

/// Adjust the horizontal position.
pub enum AlignItems {
    Start,
    Center,
    End,
}

/// Adjust the vertical position.
pub enum JustifyContent {
    Start,
    Center,
    End,
}

/// Adjust the text horizontal position.
pub enum TextAlign {
    Start,
    Center,
    End,
}

pub enum TextOverflow<'a> {
    Clip,
    Ellipsis,
    Content(&'a str),
}

pub enum Position {
    Static,
    Absolute,
}

/// Style is used by `text` or `img` element.
/// Text element is `inline-block`, so you can adjust text position by using `text_align`.
pub struct Style<'a> {
    pub margin: Margin,
    /// For Text element
    pub line_height: f32,
    /// For Text element
    pub font_size: f32,
    /// For Text element
    pub font_family: Option<Vec<u8>>,
    /// For Text element
    pub word_break: WordBreak,
    /// For Text element
    pub color: Rgba<u8>,
    pub text_align: TextAlign,
    /// For Text element
    pub max_height: Option<u32>,
    /// For Text element
    /// This property support multiline.
    pub text_overflow: TextOverflow<'a>,
    pub position: Position,
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
}

impl<'a> Default for Style<'a> {
    fn default() -> Self {
        Style {
            margin: Margin::default(),
            line_height: 1.5,
            font_size: 30.,
            font_family: None,
            word_break: WordBreak::Normal,
            color: Rgba([0, 0, 0, 255]),
            text_align: TextAlign::Start,
            max_height: None,
            text_overflow: TextOverflow::Clip,
            position: Position::Static,
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

/// Window is act like flexbox. And default direction is `column`.
/// You can adjust position with `align_item` and `justify_content`.
/// You must pass `background_image` or `background_color` for constructing surface.
pub struct WindowStyle {
    pub height: u32,
    pub width: u32,
    pub background_color: Option<Rgba<u8>>,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
}

impl Default for WindowStyle {
    fn default() -> Self {
        WindowStyle {
            height: 0,
            width: 0,
            background_color: None,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
        }
    }
}
