use image::ImageError;

use crate::Error;

use super::context::Context;
use super::element::{Element, Img, Line, Text};
use super::font::{create_font, Font, FontContext, FontIndexStore};
use super::glyph::Glyph;
use super::layout::{SplitText, TextArea};
use super::style::{Style, WindowStyle};
use std::{cell::RefCell, ops::Range, path::Path, str};

#[derive(Default)]
pub(super) struct Content {
    pub(super) height: u32,
    pub(super) width: u32,
}

pub struct Tree(pub(super) Vec<Element>);

/// This struct write text to PNG.
/// You can set text or img with `set_*` method.
/// And you can set style with `set_*_style` method.
pub struct OGImageWriter {
    pub(super) context: Context,
    pub(super) tree: Tree,
    pub(super) window: WindowStyle,
    pub(super) content: Content,
    pub(super) font_context: FontContext,
}

impl OGImageWriter {
    /// Set window style. Window act like CSS `flexbox`.
    pub fn new(window: WindowStyle) -> Result<Self, Error> {
        let context = Context::new(window.width, window.height);

        let mut this = OGImageWriter {
            context,
            tree: OGImageWriter::create_tree(),
            window,
            content: Content::default(),
            font_context: FontContext::new(),
        };

        this.process_background()?;

        Ok(this)
    }

    /// Set window style. Window act like CSS `flexbox`.
    /// Height and width are set by specified image.
    pub fn from_data(window: WindowStyle, data: &[u8]) -> Result<Self, Error> {
        let context = Context::from_data(data)?;

        let image = match &context.image {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };

        let width = image.width();
        let height = image.height();

        Ok(OGImageWriter {
            context,
            tree: OGImageWriter::create_tree(),
            window: WindowStyle {
                width,
                height,
                ..window
            },
            content: Content::default(),
            font_context: FontContext::new(),
        })
    }

    /// You can get FontContext.
    /// You can specify global fallback font by using `FontContext::push`.
    pub fn get_font_context(&mut self) -> &mut FontContext {
        &mut self.font_context
    }

    pub(super) fn create_tree() -> Tree {
        Tree(Vec::with_capacity(2))
    }

    /// Set text you want to write to image.
    /// And set the text element style. Text element act like CSS `inline-block`.
    pub fn set_text(&mut self, text: &str, style: Style, font: Vec<u8>) -> Result<(), Error> {
        let textarea = RefCell::new(TextArea::new());
        textarea.borrow_mut().push_text(text);

        let font = create_font(font)?;

        self.process_text(textarea, style, font)
    }

    /// Set [TextArea](super::TextArea) to image.
    pub fn set_textarea(
        &mut self,
        textarea: TextArea,
        style: Style,
        font: Vec<u8>,
    ) -> Result<(), Error> {
        let font = create_font(font)?;
        self.process_text(RefCell::new(textarea), style, font)
    }

    /// Set image you want to write to image. And set the image element style.
    pub fn set_img(
        &mut self,
        src: &str,
        width: u32,
        height: u32,
        style: Style,
    ) -> Result<(), Error> {
        self.process_img_with_src(src, width, height, style)
    }

    /// Set image you want to write to image. And set the image element style.
    pub fn set_img_with_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        style: Style,
    ) -> Result<(), ImageError> {
        self.process_img_with_data(data, width, height, style)
    }

    /// Set generated image by [OGImageWriter](Self) on parent image
    pub fn set_container(&mut self, writer: &mut OGImageWriter, style: Style) -> Result<(), Error> {
        writer.paint()?;

        self.process_container(writer, style)?;

        Ok(())
    }

    /// Generate your image.
    pub fn generate(&mut self, dest: &Path) -> Result<(), Error> {
        self.paint()?;

        self.context.save(dest)
    }

    pub fn paint(&mut self) -> Result<(), Error> {
        self.process();

        while let Some(elm) = self.tree.0.pop() {
            match elm {
                Element::Img(Some(img)) => self.paint_img(img)?,
                Element::Text(Some(text)) => self.paint_text(text)?,
                _ => return Err(Error::NullElement),
            }
        }

        Ok(())
    }

    pub fn into_vec(self) -> Result<Vec<u8>, Error> {
        self.context.into_vec()
    }

    fn paint_img(&mut self, img: Img) -> Result<(), Error> {
        self.context.draw_image(img.buf, img.rect.x, img.rect.y)
    }

    fn paint_text(&mut self, text_elm: Text) -> Result<(), Error> {
        fn render_text(
            text: &str,
            range: &mut Range<usize>,
            font: &Font,
            context: &mut Context,
            current_width: &mut u32,
            style: &Style,
            line: &Line,
        ) -> Result<(), Error> {
            let next_text = &text[range.clone()];

            context.draw_text(
                style.color.as_image_rgba(),
                line.rect.x + *current_width,
                line.rect.y,
                style.font_size,
                font,
                next_text,
            )?;

            *range = range.end..range.end;
            *current_width += context.text_extents(next_text, style.font_size, font).width as u32;

            Ok(())
        }

        let style = text_elm.style;
        let mut current_split_text: Option<&SplitText> = None;
        let mut current_glyph: Option<&Glyph> = None;
        for line in &text_elm.lines {
            let text = &text_elm.text[line.range.clone()];
            let mut range = 0..0;
            let mut current_width = 0;
            for (i, ch) in text.char_indices() {
                let ch_len = ch.to_string().len();
                let (split_text, glyph) = text_elm.textarea.get_glyphs_from_char_range(
                    line.range.start + i..line.range.start + i + ch_len,
                );
                let contained = match (split_text, glyph) {
                    (Some(split_text), Some(glyph)) => {
                        match (&current_split_text, &current_glyph) {
                            (Some(current_split_text), Some(current_glyph)) => {
                                split_text.range.start >= current_split_text.range.start
                                    && split_text.range.end <= current_split_text.range.end
                                    && glyph.font_index_store == current_glyph.font_index_store
                            }
                            (None, None) => {
                                current_split_text = Some(split_text);
                                current_glyph = Some(glyph);
                                true
                            }
                            _ => return Err(Error::OutOfRangeText),
                        }
                    }
                    _ => return Err(Error::OutOfRangeText),
                };

                if !contained {
                    // current_split_text is always Some.
                    let style = match current_split_text {
                        Some(current_split_text) => match &current_split_text.style {
                            Some(style) => style,
                            None => &style,
                        },
                        None => &style,
                    };

                    match current_glyph {
                        Some(glyph) => match &glyph.font_index_store {
                            FontIndexStore::Global(idx) => {
                                let store = self.font_context.borrow_font_store();
                                let store = store.borrow();
                                let font = store.borrow_font(idx);
                                render_text(
                                    text,
                                    &mut range,
                                    font,
                                    &mut self.context,
                                    &mut current_width,
                                    style,
                                    line,
                                )?;
                            }
                            FontIndexStore::Parent(_) => render_text(
                                text,
                                &mut range,
                                &text_elm.font,
                                &mut self.context,
                                &mut current_width,
                                style,
                                line,
                            )?,
                            FontIndexStore::Child(_) => match current_split_text {
                                Some(temp_split_text) => match &temp_split_text.font {
                                    Some(font) => render_text(
                                        text,
                                        &mut range,
                                        font,
                                        &mut self.context,
                                        &mut current_width,
                                        style,
                                        line,
                                    )?,
                                    None => return Err(Error::NotFoundSpecifiedFontFamily),
                                },
                                None => return Err(Error::OutOfRangeText),
                            },
                        },
                        None => return Err(Error::NotFoundSpecifiedFontFamily),
                    };

                    current_split_text = split_text;
                    current_glyph = glyph;
                }
                range.end = i + ch_len;
            }
            if !range.is_empty() {
                let style = match current_split_text {
                    Some(inner_split_text) => match &inner_split_text.style {
                        Some(style) => style,
                        None => &style,
                    },
                    None => &style,
                };

                match current_glyph {
                    Some(glyph) => match &glyph.font_index_store {
                        FontIndexStore::Global(idx) => {
                            let store = self.font_context.borrow_font_store();
                            let store = store.borrow();
                            let font = store.borrow_font(idx);
                            self.context.draw_text(
                                style.color.as_image_rgba(),
                                line.rect.x + current_width,
                                line.rect.y,
                                style.font_size,
                                font,
                                &text[range.clone()],
                            )?;
                        }
                        FontIndexStore::Parent(_) => {
                            self.context.draw_text(
                                style.color.as_image_rgba(),
                                line.rect.x + current_width,
                                line.rect.y,
                                style.font_size,
                                &text_elm.font,
                                &text[range.clone()],
                            )?;
                        }
                        FontIndexStore::Child(_) => match current_split_text {
                            Some(split_text) => match &split_text.font {
                                Some(font) => {
                                    self.context.draw_text(
                                        style.color.as_image_rgba(),
                                        line.rect.x + current_width,
                                        line.rect.y,
                                        style.font_size,
                                        font,
                                        &text[range.clone()],
                                    )?;
                                }
                                None => return Err(Error::NotFoundSpecifiedFontFamily),
                            },
                            None => return Err(Error::OutOfRangeText),
                        },
                    },
                    None => return Err(Error::OutOfRangeText),
                };
            }
        }

        Ok(())
    }
}
