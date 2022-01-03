pub use image::{Rgb, Rgba as ImageRgba};
use std::marker::Copy;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum KernSetting {
    Normal,
    Metrics,
    Optical,
}

#[derive(Debug, Copy, Clone)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    pub(super) fn as_image_rgba(&self) -> ImageRgba<u8> {
        ImageRgba(self.0)
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum WordBreak {
    Normal,
    BreakAll,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum WhiteSpace {
    Normal,
    PreLine,
}

impl WhiteSpace {
    pub(crate) fn is_pre(&self) -> bool {
        match self {
            Self::Normal => false,
            Self::PreLine => true,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Margin(pub i32, pub i32, pub i32, pub i32);

#[derive(Debug, Default, Clone, Copy)]
pub struct BorderRadius(pub u32, pub u32, pub u32, pub u32);

/// Adjust the horizontal position.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum AlignItems {
    Start,
    Center,
    End,
}

/// Adjust the vertical position.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum JustifyContent {
    Start,
    Center,
    End,
}

/// Adjust the text horizontal position.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum TextAlign {
    Start,
    Center,
    End,
}

#[derive(Debug)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
    Content(String),
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum Position {
    Static,
    Absolute,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum FlexDirection {
    Column,
    Row,
}

/// Style is used by `text` or `img` element.
/// Text element is `inline-block`, so you can adjust text position by using `text_align`.
#[derive(Debug)]
pub struct Style {
    pub margin: Margin,
    /// For Text element
    pub line_height: f32,
    /// For Text element
    pub font_size: f32,
    /// For Text element
    pub letter_spacing: i32,
    /// For Text element
    pub kern_setting: KernSetting,
    /// For Text element
    pub word_break: WordBreak,
    /// For Text element
    pub white_space: WhiteSpace,
    /// For Text element
    pub color: Rgba,
    /// For Text element
    pub text_align: TextAlign,
    /// For Text element
    pub max_height: Option<u32>,
    /// For Text element
    pub max_width: Option<u32>,
    /// For Text element
    /// This property support multiline.
    pub text_overflow: TextOverflow,
    pub position: Position,
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
    /// For Img element
    pub border_radius: BorderRadius,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            margin: Margin::default(),
            line_height: 1.5,
            font_size: 30.,
            letter_spacing: 0,
            kern_setting: KernSetting::Normal,
            word_break: WordBreak::Normal,
            white_space: WhiteSpace::Normal,
            color: Rgba([0, 0, 0, 255]),
            text_align: TextAlign::Start,
            max_height: None,
            max_width: None,
            text_overflow: TextOverflow::Clip,
            position: Position::Static,
            top: None,
            right: None,
            bottom: None,
            left: None,
            border_radius: BorderRadius::default(),
        }
    }
}

pub enum LogicalFlexRowPosition {
    Start,
    Center,
    End,
}

/// Window is act like flexbox. And default direction is `column`.
/// You can adjust position with `align_item` and `justify_content`.
/// You must pass `background_image` or `background_color` for constructing surface.
#[derive(Debug)]
pub struct WindowStyle {
    pub height: u32,
    pub width: u32,
    pub background_color: Option<Rgba>,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    /// This controls the direction in which the children of a node are laid out.
    pub flex_direction: FlexDirection,
}

impl WindowStyle {
    pub fn logical_flex_row_position(&self) -> LogicalFlexRowPosition {
        match &self.flex_direction {
            FlexDirection::Column => match self.align_items {
                AlignItems::Start => LogicalFlexRowPosition::Start,
                AlignItems::Center => LogicalFlexRowPosition::Center,
                AlignItems::End => LogicalFlexRowPosition::End,
            },
            FlexDirection::Row => match self.justify_content {
                JustifyContent::Start => LogicalFlexRowPosition::Start,
                JustifyContent::Center => LogicalFlexRowPosition::Center,
                JustifyContent::End => LogicalFlexRowPosition::End,
            },
        }
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        WindowStyle {
            height: 0,
            width: 0,
            background_color: None,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
        }
    }
}
