use super::font::{is_newline_as_whitespace, whitespace_width};
use super::img::ImageInputFormat;
use crate::renderer::{calculate_text_width, draw_text_mut, get_glyph_rect, FontSetting};
use crate::Error;
use ab_glyph::{FontArc, PxScaleFont, ScaleFont};
use image::imageops::overlay;
use image::{load_from_memory_with_format, DynamicImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::map::map_colors;
use std::path::Path;

pub use image::ImageOutputFormat;

pub(super) struct FontMetrics {
    pub height: f32,
    pub width: f32,
}

pub(super) struct Context {
    pub image: Option<RgbaImage>,
}

impl Context {
    pub fn new(w: u32, h: u32) -> Self {
        let image = RgbaImage::new(w, h);
        Self { image: Some(image) }
    }

    pub fn from_data(data: &[u8], format: ImageInputFormat) -> Result<Self, Error> {
        let image = load_from_memory_with_format(data, format.as_image_format())?;
        Ok(Self {
            image: Some(image.into_rgba8()),
        })
    }

    pub fn text_extents(
        &self,
        text: &str,
        font: PxScaleFont<&FontArc>,
        setting: &FontSetting,
    ) -> FontMetrics {
        let mut chars = text.chars().peekable();
        let mut width = 0.;
        while let Some(cur_char) = chars.next() {
            let metrics = self.char_extents(cur_char, chars.peek().copied(), font, setting);
            width += metrics.width;
        }

        FontMetrics {
            height: font.ascent() + font.descent(),
            width,
        }
    }

    pub fn char_extents(
        &self,
        cur_char: char,
        next_char: Option<char>,
        font: PxScaleFont<&FontArc>,
        setting: &FontSetting,
    ) -> FontMetrics {
        let rect = get_glyph_rect(cur_char, &font, setting);

        let height = font.ascent() + font.descent();

        if cur_char.is_whitespace() || is_newline_as_whitespace(setting.is_pre, cur_char, next_char)
        {
            return FontMetrics {
                height,
                width: whitespace_width(setting.size),
            };
        }

        FontMetrics {
            height,
            width: match rect {
                Some(rect) => {
                    calculate_text_width(cur_char, next_char, &font, &rect, setting) as f32
                }
                None => 0.,
            },
        }
    }

    pub fn draw_background_color(&mut self, rgba: Rgba<u8>) -> Result<(), Error> {
        let image = match &self.image {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };
        self.image = Some(map_colors(image, |_| rgba));

        Ok(())
    }

    pub fn draw_image(
        &mut self,
        buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
    ) -> Result<(), Error> {
        let image = match &mut self.image {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };
        overlay(image, &buf, x, y);

        Ok(())
    }

    pub fn draw_text(
        &mut self,
        color: Rgba<u8>,
        x: u32,
        y: u32,
        font: &FontArc,
        setting: &FontSetting,
        text: &str,
    ) -> Result<(), Error> {
        let image = match &mut self.image {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };
        draw_text_mut(image, color, x, y, font, setting, text);

        Ok(())
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        match &self.image {
            Some(image) => {
                image.save(path)?;
                Ok(())
            }
            None => Err(Error::NotFoundContainerImage),
        }
    }

    pub(super) fn into_vec(mut self) -> Result<Vec<u8>, Error> {
        match self.image.take() {
            None => Err(Error::NullElement),
            Some(img) => Ok(img.into_vec()),
        }
    }

    pub(super) fn encode(mut self, f: ImageOutputFormat) -> Result<Vec<u8>, Error> {
        match self.image.take() {
            None => Err(Error::NullElement),
            Some(img) => {
                let mut buf = vec![];
                DynamicImage::ImageRgba8(img).write_to(&mut buf, f)?;
                Ok(buf)
            }
        }
    }
}
