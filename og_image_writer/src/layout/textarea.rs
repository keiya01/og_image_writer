use crate::style::Style;
use crate::Error;
use rusttype::Font;
use std::{ops::Range, str};

#[derive(Debug)]
pub(crate) struct SplitText<'a> {
    pub(crate) text: &'a str,
    pub(crate) style: Option<Style<'a>>,
    pub(crate) font: Option<Font<'a>>,
    pub(crate) range: Range<usize>,
}

#[derive(Debug, Default)]
pub struct TextArea<'a>(pub(super) Vec<SplitText<'a>>);

impl<'a> TextArea<'a> {
    pub fn new() -> TextArea<'a> {
        TextArea(vec![])
    }

    pub fn push(
        &mut self,
        text: &'a str,
        style: Style<'a>,
        font: Option<Vec<u8>>,
    ) -> Result<(), Error> {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let font = match font {
            Some(font) => Some(match Font::try_from_vec(font) {
                Some(font) => font,
                None => return Err(Error::InvalidFontBytes),
            }),
            None => None,
        };

        let split_text = SplitText {
            text,
            style: Some(style),
            font,
            range: last_range_end..last_range_end + text.len(),
        };

        self.0.push(split_text);

        Ok(())
    }

    pub fn push_text(&mut self, text: &'a str) {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };
        let split_text = SplitText {
            text,
            style: None,
            font: None,
            range: last_range_end..last_range_end + text.len(),
        };

        self.0.push(split_text);
    }

    pub(super) fn as_string(&self) -> String {
        let mut text = String::new();
        for split_text in &self.0 {
            text.push_str(split_text.text);
        }
        text
    }

    pub(crate) fn get_split_text_from_char_range(&self, range: Range<usize>) -> Option<&SplitText> {
        for split_text in &self.0 {
            if split_text.range.start <= range.start && range.end <= split_text.range.end {
                return Some(split_text);
            }
        }
        None
    }
}
