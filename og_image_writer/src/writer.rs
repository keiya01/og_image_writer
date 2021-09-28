use image::ImageError;

use crate::Error;

use super::context::Context;
use super::element::{Element, Img, Text};
use super::style::{Style, WindowStyle};
use std::{path::Path, str};

#[derive(Default)]
pub(super) struct Content {
    pub(super) height: u32,
    pub(super) width: u32,
}

/// This struct write text to PNG.
/// You can set text or img with `set_*` method.
/// And you can set style with `set_*_style` method.
pub struct OGImageWriter<'a> {
    pub(super) context: Context,
    pub(super) tree: Vec<Element<'a>>,
    pub(super) window: WindowStyle,
    pub(super) content: Content,
}

impl<'a> OGImageWriter<'a> {
    /// Set window style. Window act like CSS `flexbox`.
    pub fn new(window: WindowStyle) -> Result<Self, Error> {
        let context = Context::new(window.width, window.height);

        let mut this = OGImageWriter {
            context,
            tree: OGImageWriter::create_tree(),
            window,
            content: Content::default(),
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
        })
    }

    pub(super) fn create_tree() -> Vec<Element<'a>> {
        Vec::with_capacity(2)
    }

    /// Set text you want to write to image.
    /// And set the text element style. Text element act like CSS `inline-block`.
    pub fn set_text(
        &mut self,
        text: &'a str,
        style: Style<'a>,
        font: Vec<u8>,
    ) -> Result<(), Error> {
        self.process_text(text, style, font)
    }

    /// Set image you want to write to image. And set the image element style.
    pub fn set_img(
        &mut self,
        src: &'a str,
        width: u32,
        height: u32,
        style: Style<'a>,
    ) -> Result<(), Error> {
        self.process_img_with_src(src, width, height, style)
    }

    /// Set image you want to write to image. And set the image element style.
    pub fn set_img_with_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        style: Style<'a>,
    ) -> Result<(), ImageError> {
        self.process_img_with_data(data, width, height, style)
    }

    pub fn set_container(
        &mut self,
        writer: &mut OGImageWriter,
        style: Style<'a>,
    ) -> Result<(), Error> {
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

        while let Some(elm) = self.tree.pop() {
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

    fn paint_text(&mut self, text_elm: Text<'a>) -> Result<(), Error> {
        let style = text_elm.style;
        for line in &text_elm.lines {
            self.context.draw_text(
                style.color,
                line.rect.x,
                line.rect.y,
                style.font_size,
                &text_elm.font,
                &text_elm.text[line.range.clone()],
            )?;
        }

        Ok(())
    }
}
