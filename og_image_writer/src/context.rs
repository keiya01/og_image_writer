use image::imageops::overlay;
use image::{load_from_memory, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::map::map_colors;
use rusttype::{Font, Scale, IntoGlyphId};
use std::path::Path;

use crate::Error;

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

    pub fn from_data(data: &[u8]) -> Result<Self, Error> {
        let image = load_from_memory(data)?;
        Ok(Self {
            image: Some(image.into_rgba8()),
        })
    }

    pub fn text_extents(&self, text: &str, size: f32, font: &Font) -> FontMetrics {
        let glyphs = font.glyphs_for(text.chars());
        let scale = Scale::uniform(size);
        let vmetrics = font.v_metrics(scale);

        let mut width = 0.;
        for g in glyphs {
            let sg = g.scaled(scale);
            let hmetrics = sg.h_metrics();
            width += hmetrics.advance_width;
        }

        FontMetrics {
            height: vmetrics.ascent + vmetrics.descent,
            width,
        }
    }

    pub fn char_extents(&self, ch: char, size: f32, font: &Font) -> FontMetrics {
        let glyph_id = ch.into_glyph_id(font);
        let glyph = font.glyph(glyph_id);
        let scale = Scale::uniform(size);
        let vmetrics = font.v_metrics(scale);

        let sg = glyph.scaled(scale);
        let hmetrics = sg.h_metrics();

        FontMetrics {
            height: vmetrics.ascent + vmetrics.descent,
            width: hmetrics.advance_width,
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
        size: f32,
        font: &Font,
        text: &str,
    ) -> Result<(), Error> {
        let image = match &mut self.image {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };
        draw_text_mut(image, color, x, y, Scale::uniform(size), font, text);

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
}
