use std::ops::Range;

use crate::char::RenderingCharIndices;
use crate::font::{match_font_family, FontArc};
use crate::font_context::{FontContext, FontIndex, FontIndexStore};
use crate::font_trait::Font;
use crate::glyph::Glyph;
use crate::style::Style;
use crate::Error;

#[derive(Debug)]
pub(crate) struct SplitText {
    pub(crate) text: String,
    pub(crate) style: Option<Style>,
    pub(crate) font: Option<FontArc>,
    // Fast path for glyphs.
    pub(crate) range: Range<usize>,
    pub(crate) glyphs: Vec<Glyph>,
}

impl SplitText {
    // Set bundled glyphs with text range that has same font.
    // When all text have same font then glyphs length is 1.
    // Glyph has text range bundled with same font.
    pub(super) fn set_glyphs(
        &mut self,
        parent_font: &Option<impl Font>,
        current_range_start: &mut usize,
        font_context: &FontContext,
    ) -> Result<(), Error> {
        let mut glyphs = vec![];
        // TODO: Handle parent font as Vec
        let parent_font_index = 0;

        let text = &self.text;
        let child_font = &self.font;
        // TODO: Handle child font as Vec
        let child_font_index = 0;

        let mut current_range_end = *current_range_start;

        #[allow(unused_assignments)]
        let mut font_index_store: Option<FontIndexStore> = None;
        let mut prev_font_index_store: Option<FontIndexStore> = None;

        for (_, _, ch, _) in RenderingCharIndices::from_str(text) {
            let has_parent_font = match parent_font {
                Some(parent_font) => match_font_family(ch, parent_font),
                None => false,
            };
            let has_child_font = match child_font {
                Some(font) => match_font_family(ch, font),
                None => false,
            };

            if has_child_font {
                font_index_store = Some(FontIndexStore::Child(FontIndex(child_font_index)));
            } else if has_parent_font {
                font_index_store = Some(FontIndexStore::Parent(FontIndex(parent_font_index)));
            } else {
                font_index_store =
                    Some(FontIndexStore::Global(font_context.select_font_family(ch)?));
            }

            let is_equal_font_index_store = match (&font_index_store, &prev_font_index_store) {
                (Some(current), Some(prev)) => current == prev,
                (Some(_), None) => {
                    prev_font_index_store = font_index_store.clone();
                    true
                }
                _ => false,
            };

            if !is_equal_font_index_store {
                let idx = match prev_font_index_store.take() {
                    Some(store) => store,
                    None => return Err(Error::NotFoundSpecifiedFontFamily),
                };
                glyphs.push(Glyph::new(*current_range_start..current_range_end, idx));
                prev_font_index_store = font_index_store;
                *current_range_start = current_range_end;
            }

            current_range_end += ch.to_string().len();
        }

        let font_index_store = match prev_font_index_store.take() {
            Some(store) => store,
            None => {
                self.glyphs.append(&mut glyphs);
                *current_range_start = current_range_end;
                return Ok(());
            }
        };

        glyphs.push(Glyph::new(
            *current_range_start..current_range_end,
            font_index_store,
        ));
        *current_range_start = current_range_end;

        self.glyphs.append(&mut glyphs);

        Ok(())
    }

    pub(crate) fn get_glyphs_from_char_range(&self, range: Range<usize>) -> Option<&Glyph> {
        for glyph in &self.glyphs {
            if glyph.range.start <= range.start && range.end <= glyph.range.end {
                return Some(glyph);
            }
        }
        None
    }
}
