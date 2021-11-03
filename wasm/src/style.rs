use og_image_writer::style::{
    AlignItems, BorderRadius, FlexDirection, JustifyContent, Margin, Position, Rgba, Style,
    TextAlign, TextOverflow, WindowStyle, WordBreak,
};
use std::marker::Copy;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_DEFINITION: &'static str = r#"
type TextOverflow = 'clip' | 'ellipsis' | string;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "TextOverflow")]
    pub type JsTextOverflow;
}

#[wasm_bindgen(js_name = Rgba)]
#[derive(Copy, Clone)]
pub struct JsRgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen(js_class = Rgba)]
impl JsRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> JsRgba {
        JsRgba { r, g, b, a }
    }
}

#[wasm_bindgen(js_name = BorderRadius)]
#[derive(Default, Copy, Clone)]
pub struct JsBorderRadius {
    pub top_left: u32,
    pub top_right: u32,
    pub bottom_left: u32,
    pub bottom_right: u32,
}

#[wasm_bindgen(js_class = BorderRadius)]
impl JsBorderRadius {
    pub fn new(tl: u32, tr: u32, bl: u32, br: u32) -> JsBorderRadius {
        JsBorderRadius {
            top_left: tl,
            top_right: tr,
            bottom_left: bl,
            bottom_right: br,
        }
    }
}

#[wasm_bindgen(js_name = Margin)]
#[derive(Default, Copy, Clone)]
pub struct JsMargin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[wasm_bindgen(js_class = Margin)]
impl JsMargin {
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> JsMargin {
        JsMargin {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[wasm_bindgen(js_name = Style)]
pub struct JsStyle {
    pub margin: JsMargin,
    /// For Text element
    pub line_height: f32,
    /// For Text element
    pub font_size: f32,
    /// For Text element
    pub word_break: WordBreak,
    /// For Text element
    pub color: JsRgba,
    /// For Text element
    pub text_align: TextAlign,
    /// For Text element
    pub max_height: Option<u32>,
    /// For Text element
    pub max_width: Option<u32>,
    /// For Text element
    /// This property support multiline.
    text_overflow: String,
    pub position: Position,
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
    /// For Img element
    pub border_radius: JsBorderRadius,
}

#[wasm_bindgen(js_class = Style)]
impl JsStyle {
    pub fn new() -> JsStyle {
        JsStyle::default()
    }

    #[wasm_bindgen(setter)]
    pub fn set_text_overflow(&mut self, value: String) {
        self.text_overflow = value;
    }
}

impl Default for JsStyle {
    fn default() -> Self {
        JsStyle {
            margin: JsMargin::default(),
            line_height: 1.5,
            font_size: 30.,
            word_break: WordBreak::Normal,
            color: JsRgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            text_align: TextAlign::Start,
            max_height: None,
            max_width: None,
            text_overflow: "clip".to_string(),
            position: Position::Static,
            top: None,
            right: None,
            bottom: None,
            left: None,
            border_radius: JsBorderRadius::default(),
        }
    }
}

#[wasm_bindgen(js_name = WindowStyle)]
pub struct JsWindowStyle {
    pub height: u32,
    pub width: u32,
    pub background_color: Option<JsRgba>,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    /// This controls the direction in which the children of a node are laid out.
    pub flex_direction: FlexDirection,
}

#[wasm_bindgen(js_class = WindowStyle)]
impl JsWindowStyle {
    pub fn new() -> JsWindowStyle {
        JsWindowStyle::default()
    }
}

impl Default for JsWindowStyle {
    fn default() -> Self {
        JsWindowStyle {
            height: 0,
            width: 0,
            background_color: None,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
        }
    }
}

pub fn from_js_style(style: JsStyle) -> Style {
    Style {
        margin: Margin(
            style.margin.top,
            style.margin.right,
            style.margin.bottom,
            style.margin.left,
        ),
        line_height: style.line_height,
        font_size: style.font_size,
        word_break: style.word_break,
        color: Rgba([style.color.r, style.color.g, style.color.b, style.color.a]),
        text_align: style.text_align,
        max_height: style.max_height,
        max_width: style.max_width,
        text_overflow: {
            match &style.text_overflow[..] {
                "clip" => TextOverflow::Clip,
                "ellipsis" => TextOverflow::Ellipsis,
                _ => TextOverflow::Content(style.text_overflow.clone()),
            }
        },
        position: style.position,
        top: style.top,
        right: style.right,
        bottom: style.bottom,
        left: style.left,
        border_radius: BorderRadius(
            style.border_radius.top_left,
            style.border_radius.top_right,
            style.border_radius.bottom_left,
            style.border_radius.bottom_right,
        ),
    }
}

pub fn from_js_window_style(style: JsWindowStyle) -> WindowStyle {
    WindowStyle {
        height: style.height,
        width: style.width,
        background_color: style
            .background_color
            .map(|color| Rgba([color.r, color.g, color.b, color.a])),
        align_items: style.align_items,
        justify_content: style.justify_content,
        flex_direction: style.flex_direction,
    }
}
