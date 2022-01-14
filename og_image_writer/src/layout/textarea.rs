use crate::char::{CharFlags, RenderingCharIndices};
use crate::font::{
    create_font, match_font_family, FontArc, FontContext, FontIndex, FontIndexStore, FontMetrics,
};
use crate::font_trait::Font;
use crate::glyph::Glyph;
use crate::renderer::FontSetting;
use crate::style::Style;
use crate::Error;
use std::{ops::Range, str};

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
    fn set_glyphs(
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

/// TextArea is box to store each text with style.
/// For example you can set style to text one by one.
#[derive(Debug, Default)]
pub struct TextArea(pub(super) Vec<SplitText>);

impl TextArea {
    pub fn new() -> TextArea {
        TextArea(vec![])
    }

    /// Push text with style.
    pub fn push(&mut self, text: &str, style: Style, font: Option<Vec<u8>>) -> Result<(), Error> {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let font = match font {
            Some(font) => Some(match create_font(font) {
                Ok(font) => font,
                Err(_) => return Err(Error::InvalidFontBytes),
            }),
            None => None,
        };

        let mut string = String::new();
        string.push_str(text);

        let split_text = SplitText {
            text: string,
            style: Some(style),
            font,
            range: last_range_end..last_range_end + text.len(),
            glyphs: vec![],
        };

        self.0.push(split_text);

        Ok(())
    }

    /// Push text without style.
    /// Style is override with parent style.
    /// Parent style is set with [`OGImageWriter::set_textarea()`](crate::writer::OGImageWriter::set_textarea).
    pub fn push_text(&mut self, text: &str) {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let mut string = String::new();
        string.push_str(text);

        let split_text = SplitText {
            text: string,
            style: None,
            font: None,
            range: last_range_end..last_range_end + text.len(),
            glyphs: vec![],
        };

        self.0.push(split_text);
    }

    pub(super) fn push_text_with_glyphs(
        &mut self,
        text: &str,
        font: &Option<FontArc>,
        font_context: &FontContext,
    ) -> Result<(), Error> {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let mut string = String::new();
        string.push_str(text);

        let mut split_text = SplitText {
            text: string,
            style: None,
            font: None,
            range: last_range_end..last_range_end + text.len(),
            glyphs: vec![],
        };

        let mut current_range_start = last_range_end;

        split_text.set_glyphs(font, &mut current_range_start, font_context)?;

        self.0.push(split_text);

        Ok(())
    }

    pub(super) fn as_string(&self) -> String {
        let mut text = String::new();
        for split_text in &self.0 {
            text.push_str(&split_text.text);
        }
        text
    }

    pub(crate) fn get_glyphs_from_char_range(
        &self,
        range: Range<usize>,
    ) -> (Option<&SplitText>, Option<&Glyph>) {
        for split_text in &self.0 {
            let glyph = split_text.get_glyphs_from_char_range(range.clone());
            if glyph.is_some() {
                return (Some(split_text), glyph);
            }
        }
        (None, None)
    }

    pub(crate) fn set_glyphs(
        &mut self,
        parent_font: &Option<impl Font>,
        font_context: &FontContext,
    ) -> Result<(), Error> {
        let mut current_range_start = 0;
        for split_text in self.0.iter_mut() {
            split_text.set_glyphs(parent_font, &mut current_range_start, font_context)?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn char_extents(
        &self,
        cur_char: char,
        next_char: Option<char>,
        flags: &Option<CharFlags>,
        parent_font: &dyn Font,
        range: Range<usize>,
        font_context: &FontContext,
        setting: &FontSetting,
    ) -> Result<FontMetrics, Error> {
        let extents = match self.get_glyphs_from_char_range(range) {
            (Some(split_text), Some(glyph)) => {
                let setting = match &split_text.style {
                    Some(style) => FontSetting {
                        size: style.font_size,
                        letter_spacing: style.letter_spacing,
                        kern_setting: style.kern_setting,
                        is_pre: style.white_space.is_pre(),
                    },
                    None => setting.clone(),
                };
                match &glyph.font_index_store {
                    FontIndexStore::Global(idx) => font_context.with(idx, |font| {
                        font.char_extents(cur_char, next_char, flags, &setting)
                    }),
                    FontIndexStore::Parent(_) => {
                        parent_font.char_extents(cur_char, next_char, flags, &setting)
                    }
                    FontIndexStore::Child(_) => match &split_text.font {
                        Some(font) => font.char_extents(cur_char, next_char, flags, &setting),
                        None => return Err(Error::NotFoundSpecifiedFontFamily),
                    },
                }
            }
            _ => return Err(Error::OutOfRangeText),
        };

        Ok(extents)
    }
}
