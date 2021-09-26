use image::imageops::overlay;
use image::{load_from_memory, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::map::map_colors;
use rusttype::{Font, Scale};
use std::path::Path;

pub(super) struct FontMetrics {
    pub height: f32,
    pub width: f32,
}

pub(super) struct Context {
    pub image: RgbaImage,
}

impl Context {
    pub fn new(w: u32, h: u32) -> Self {
        let image = RgbaImage::new(w, h);
        Self { image }
    }

    pub fn from_data(data: &[u8]) -> Self {
        let image = load_from_memory(data).expect("Could not load image");
        Self {
            image: image.into_rgba8(),
        }
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

    pub fn draw_background_color(&mut self, rgba: Rgba<u8>) {
        self.image = map_colors(&self.image, |_| rgba);
    }

    pub fn draw_image(&mut self, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32) {
        overlay(&mut self.image, &buf, x, y);
    }

    pub fn draw_text(
        &mut self,
        color: Rgba<u8>,
        x: u32,
        y: u32,
        size: f32,
        font: &Font,
        text: &str,
    ) {
        draw_text_mut(
            &mut self.image,
            color,
            x,
            y,
            Scale::uniform(size),
            font,
            text,
        );
    }

    pub fn save(&self, path: &Path) {
        let _ = self.image.save(path).expect("Could not save image");
    }
}
