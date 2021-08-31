pub use cairo::{FontSlant as FontStyle, FontWeight};

pub enum WordBreak {
    Normal,
    BreakAll,
}

pub struct RGB(pub f64, pub f64, pub f64);

#[derive(Default)]
pub struct Margin(pub f64, pub f64, pub f64, pub f64);

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

/// Style is used by `text` or `img` element.
/// Text element is `inline-block`, so you can adjust text position by using `text_align`.
pub struct Style<'a> {
    pub margin: Margin,
    pub line_height: f64,
    pub font_size: f64,
    pub font_family: &'a str,
    pub font_style: FontStyle,
    pub font_weight: FontWeight,
    pub word_break: WordBreak,
    pub color: RGB,
    pub text_align: TextAlign,
}

impl<'a> Default for Style<'a> {
    fn default() -> Self {
        Style {
            margin: Margin::default(),
            line_height: 1.5,
            font_size: 30.,
            font_family: "",
            font_style: FontStyle::Normal,
            font_weight: FontWeight::Bold,
            word_break: WordBreak::Normal,
            color: RGB(0., 0., 0.),
            text_align: TextAlign::Start,
        }
    }
}

/// Window is act like flexbox. And default direction is `column`.
/// You can adjust position with `align_item` and `justify_content`.
/// You must pass `background_image` or `background_color` for constructing surface.
pub struct WindowStyle<'a> {
    pub height: i32,
    pub width: i32,
    pub background_image: Option<&'a str>,
    pub background_color: Option<RGB>,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
}

impl<'a> Default for WindowStyle<'a> {
    fn default() -> Self {
        WindowStyle {
            height: 0,
            width: 0,
            background_image: None,
            background_color: None,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
        }
    }
}
