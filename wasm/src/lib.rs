mod style;

use og_image_writer::{style::Style, writer::OGImageWriter, Error, TextArea};
use std::panic;
use std::path::Path;
use wasm_bindgen::prelude::*;

use style::{from_js_style, from_js_window_style, JsStyle, JsWindowStyle};

struct JsSplitText {
    text: String,
    style: Option<Style>,
    font: Option<Vec<u8>>,
}

#[wasm_bindgen(start)]
pub fn wasm_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen(js_name = TextArea)]
#[derive(Default)]
pub struct JsTextArea(Vec<JsSplitText>);

#[wasm_bindgen(js_class = TextArea)]
impl JsTextArea {
    pub fn new() -> JsTextArea {
        console_error_panic_hook::set_once();
        JsTextArea::default()
    }

    pub fn push(&mut self, text: String, style: Option<JsStyle>, font: Option<Vec<u8>>) {
        let style = style.map(from_js_style);
        self.0.push(JsSplitText { text, style, font });
    }

    fn into_textarea(self) -> Result<TextArea, Error> {
        let mut textarea = TextArea::new();
        for mut split_text in self.0 {
            if let Some(style) = split_text.style.take() {
                let font = split_text.font.take();
                textarea.push(&split_text.text, style, font)?;
            } else {
                textarea.push_text(&split_text.text);
            }
        }
        Ok(textarea)
    }
}

#[wasm_bindgen(js_name = OGImageWriter)]
pub struct JsOGImageWriter {
    writer: OGImageWriter,
}

#[wasm_bindgen(js_class = OGImageWriter)]
impl JsOGImageWriter {
    pub fn new(style: JsWindowStyle) -> Self {
        let style = from_js_window_style(style);

        JsOGImageWriter {
            writer: OGImageWriter::new(style).unwrap(),
        }
    }

    pub fn from_data(style: JsWindowStyle, data: Vec<u8>) -> Self {
        let style = from_js_window_style(style);

        JsOGImageWriter {
            writer: OGImageWriter::from_data(style, &data).unwrap(),
        }
    }

    pub fn set_text(&mut self, text: String, style: JsStyle, font: Vec<u8>) {
        let style = from_js_style(style);
        self.writer.set_text(&text, style, font).unwrap();
    }

    pub fn set_textarea(&mut self, textarea: JsTextArea, style: JsStyle, font: Vec<u8>) {
        let style = from_js_style(style);
        self.writer
            .set_textarea(textarea.into_textarea().unwrap(), style, font)
            .unwrap();
    }

    pub fn set_img_with_data(&mut self, data: Vec<u8>, width: u32, height: u32, style: JsStyle) {
        let style = from_js_style(style);
        self.writer
            .set_img_with_data(&data, width, height, style)
            .unwrap();
    }

    pub fn set_container(&mut self, writer: &mut JsOGImageWriter, style: JsStyle) {
        let style = from_js_style(style);
        self.writer
            .set_container(&mut writer.writer, style)
            .unwrap();
    }

    pub fn generate(&mut self, dest: String) {
        self.writer.generate(Path::new(&dest)).unwrap();
    }

    pub fn paint(&mut self) {
        self.writer.paint().unwrap();
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.writer.into_vec().unwrap()
    }
}
