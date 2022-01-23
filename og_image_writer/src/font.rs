use super::font_trait::Font;
use super::Error;
use ab_glyph::{
    Font as AbFont, FontArc as AbFontArc, Glyph, GlyphId, OutlinedGlyph, ScaleFont as AbScaleFont,
};

pub(super) struct FontMetrics {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug)]
pub(super) struct FontArc(AbFontArc);

impl Font for FontArc {
    fn glyph_id(&self, ch: char) -> GlyphId {
        self.0.glyph_id(ch)
    }

    fn ascent(&self, scale: f32) -> f32 {
        self.0.as_scaled(scale).ascent()
    }

    fn descent(&self, scale: f32) -> f32 {
        self.0.as_scaled(scale).descent()
    }

    fn h_advance(&self, glyph_id: GlyphId, scale: f32) -> f32 {
        self.0.as_scaled(scale).h_advance(glyph_id)
    }

    fn kern(&self, first: GlyphId, second: GlyphId, scale: f32) -> f32 {
        self.0.as_scaled(scale).kern(first, second)
    }

    fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph> {
        self.0.as_scaled(scale).outline_glyph(glyph)
    }
}

pub(super) const WHITESPACE_EM: f32 = 0.2;

pub(super) fn create_font(data: Vec<u8>) -> Result<FontArc, Error> {
    match AbFontArc::try_from_vec(data) {
        Ok(font) => Ok(FontArc(font)),
        Err(_) => Err(Error::InvalidFontBytes),
    }
}

pub(super) fn match_font_family(ch: char, font: &dyn Font) -> bool {
    font.glyph_id(ch).0 != 0
}

pub(super) fn whitespace_width(size: f32) -> f32 {
    size * WHITESPACE_EM
}

#[cfg(test)]
pub(crate) mod test_utils {
    use std::collections::HashMap;

    use super::*;
    use ab_glyph::{Outline, Point, PxScaleFactor, Rect};

    type GlyphTable = HashMap<String, GlyphId>;

    #[derive(Clone, Debug)]
    pub(crate) struct FontMock {
        glyph_table: Option<GlyphTable>,
    }

    impl FontMock {
        pub(crate) fn new(supported_glyphs: Option<&str>) -> Self {
            match supported_glyphs {
                Some(supported_glyphs) => {
                    let mut glyph_table: GlyphTable = HashMap::new();
                    supported_glyphs.chars().enumerate().for_each(|(i, ch)| {
                        glyph_table.insert(ch.to_string(), GlyphId((i + 1) as u16));
                    });
                    FontMock {
                        glyph_table: Some(glyph_table),
                    }
                }
                None => FontMock { glyph_table: None },
            }
        }
    }

    impl Font for FontMock {
        fn glyph_id(&self, ch: char) -> GlyphId {
            let glyph_table = match &self.glyph_table {
                Some(glyph_table) => glyph_table,
                None => return GlyphId(1),
            };

            match glyph_table.get(&ch.to_string()[..]) {
                Some(g) => *g,
                None => GlyphId(0),
            }
        }

        fn ascent(&self, scale: f32) -> f32 {
            scale / 2.
        }

        fn descent(&self, scale: f32) -> f32 {
            -(scale / 2.)
        }

        fn h_advance(&self, _glyph_id: GlyphId, scale: f32) -> f32 {
            scale
        }

        fn kern(&self, _first: GlyphId, _second: GlyphId, _scale: f32) -> f32 {
            1.
        }

        fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph> {
            let point = Point { x: 0., y: 0. };
            let og = OutlinedGlyph::new(
                glyph,
                Outline {
                    bounds: Rect {
                        min: point,
                        max: point,
                    },
                    curves: Vec::new(),
                },
                PxScaleFactor {
                    horizontal: scale,
                    vertical: scale,
                },
            );
            Some(og)
        }
    }
}
