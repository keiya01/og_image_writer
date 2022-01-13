use ab_glyph::{Glyph, GlyphId, OutlinedGlyph};

pub trait Font {
    fn glyph_id(&self, ch: char) -> GlyphId;
    fn ascent(&self, scale: f32) -> f32;
    fn descent(&self, scale: f32) -> f32;
    fn h_advance(&self, glyph_id: GlyphId, scale: f32) -> f32;
    fn kern(&self, first: GlyphId, second: GlyphId, scale: f32) -> f32;
    fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph>;
}
