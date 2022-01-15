use super::char::{is_newline_as_whitespace, CharFlags, RenderingCharIndices};
use super::font::{whitespace_width, FontMetrics};
use super::renderer::FontSetting;
use crate::style::KernSetting;
use ab_glyph::{point, Glyph, GlyphId, OutlinedGlyph, Rect};

pub(super) trait Font {
    fn glyph_id(&self, ch: char) -> GlyphId;
    fn ascent(&self, scale: f32) -> f32;
    fn descent(&self, scale: f32) -> f32;
    fn h_advance(&self, glyph_id: GlyphId, scale: f32) -> f32;
    fn kern(&self, first: GlyphId, second: GlyphId, scale: f32) -> f32;
    fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph>;

    fn text_extents(&self, text: &str, setting: &FontSetting) -> FontMetrics {
        let mut chars = RenderingCharIndices::from_str(text);
        let mut width = 0.;
        while let Some((flags, _, ch, _)) = chars.next() {
            let metrics = self.char_extents(ch, chars.peek_char(), &flags, setting);
            width += metrics.width;
        }

        FontMetrics {
            height: self.ascent(setting.size) + self.descent(setting.size),
            width,
        }
    }

    fn char_extents(
        &self,
        cur_char: char,
        next_char: Option<char>,
        flags: &Option<CharFlags>,
        setting: &FontSetting,
    ) -> FontMetrics {
        let rect = self.get_glyph_rect(cur_char, setting);

        let height = self.ascent(setting.size) + self.descent(setting.size);

        if cur_char.is_whitespace() {
            return FontMetrics {
                height,
                width: whitespace_width(setting.size),
            };
        }

        FontMetrics {
            height,
            width: match rect {
                Some(rect) => {
                    self.calculate_text_width(cur_char, next_char, flags, &rect, setting) as f32
                }
                None => 0.,
            },
        }
    }

    fn get_glyph_rect(&self, ch: char, setting: &FontSetting) -> Option<Rect> {
        let glyph_id = self.glyph_id(ch);
        let q_glyph: Glyph = glyph_id.with_scale_and_position(setting.size, point(0., 0.));
        if let Some(q) = self.outline_glyph(q_glyph, setting.size) {
            return Some(q.px_bounds());
        }
        None
    }

    fn calculate_text_width(
        &self,
        cur_char: char,
        next_char: Option<char>,
        flags: &Option<CharFlags>,
        rect: &Rect,
        setting: &FontSetting,
    ) -> i32 {
        let glyph_id = self.glyph_id(cur_char);

        if cur_char.is_whitespace() || is_newline_as_whitespace(setting.is_pre, flags) {
            return whitespace_width(setting.size) as i32 + setting.letter_spacing;
        }

        let width = match setting.kern_setting {
            KernSetting::Normal => self.h_advance(glyph_id, setting.size) as i32,
            KernSetting::Optical => rect.width() as i32,
            KernSetting::Metrics => match next_char {
                Some(next) => {
                    let kern = self.kern(glyph_id, self.glyph_id(next), setting.size);
                    if kern == 0. {
                        self.h_advance(glyph_id, setting.size) as i32
                    } else {
                        (rect.width() + kern) as i32
                    }
                }
                None => self.h_advance(glyph_id, setting.size) as i32,
            },
        };
        width + setting.letter_spacing
    }
}
