use crate::style::Style;
use crate::Error;
use rusttype::Font;
use std::{ops::Range, str};

pub(crate) struct SplitText<'a> {
    pub(crate) text: &'a str,
    pub(crate) style: Option<Style<'a>>,
    pub(crate) font: Option<Font<'a>>,
    pub(crate) range: Range<usize>,
}

pub struct TextArea<'a>(Vec<SplitText<'a>>);

impl<'a> TextArea<'a> {
    pub(crate) fn new() -> TextArea<'a> {
        TextArea(vec![])
    }

    pub(crate) fn push(
        &mut self,
        text: &'a str,
        style: Style<'a>,
        font: Vec<u8>,
    ) -> Result<(), Error> {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };

        let font = match Font::try_from_vec(font) {
            Some(font) => font,
            None => return Err(Error::InvalidFontBytes),
        };

        let split_text = SplitText {
            text,
            style: Some(style),
            font: Some(font),
            range: last_range_end..text.len(),
        };

        self.0.push(split_text);

        Ok(())
    }

    pub(crate) fn push_text(&mut self, text: &'a str) {
        let last_range_end = match self.0.iter().last() {
            Some(split) => split.range.end,
            None => 0,
        };
        let split_text = SplitText {
            text,
            style: None,
            font: None,
            range: last_range_end..text.len(),
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

    // 引数のrangeとself.rangeを比較して範囲内であればsplit_textを返し、範囲外の場合は何も返さない
    // 1文字毎のrangeが渡ってきてそれに対応するsplit_textを返すイメージ
    pub(crate) fn get_split_text_from_char_range(
        &self,
        range: Range<usize>,
    ) -> Result<&SplitText, Error> {
        for split_text in &self.0 {
            if split_text.range.start <= range.start && range.end <= split_text.range.end {
                return Ok(split_text);
            }
        }
        Err(Error::OutOfRangeText)
    }
}
