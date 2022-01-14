use super::font_trait::Font;
use super::img::ImageInputFormat;
use crate::renderer::{draw_text_mut, FontSetting};
use crate::Error;
use image::imageops::overlay;
use image::{load_from_memory_with_format, DynamicImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::map::map_colors;
use std::path::Path;

pub use image::ImageOutputFormat;

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
        font: &dyn Font,
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

    pub(super) fn into_rgba(mut self) -> Result<RgbaImage, Error> {
        match self.image.take() {
            None => Err(Error::NullElement),
            Some(img) => Ok(img),
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
