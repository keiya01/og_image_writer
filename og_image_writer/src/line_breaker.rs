use super::char::{CharFlags, RenderingCharIndices};
use super::layout::TextArea;
use crate::font::{match_font_family, whitespace_width, FontContext, FontMetrics};
use crate::font_trait::Font;
use crate::renderer::FontSetting;
use crate::style::{Style, WordBreak};
use crate::Error;
use std::ops::Range;

pub(super) struct Line {
    pub(super) range: Range<usize>,
    pub(super) width: f32,
    pub(super) height: f32,
}

impl Line {
    fn new(range: Range<usize>, width: f32, height: f32) -> Self {
        assert!(!range.is_empty());

        Line {
            range,
            width,
            height,
        }
    }
}

pub(super) struct LineBreaker<'a> {
    pub(super) title: &'a str,
    pub(super) lines: Vec<Line>,
    pub(super) max_line_height: f32,
    pub(super) max_line_width: f32,
}

// TODO: support truncate text when overflow specified height.
impl<'a> LineBreaker<'a> {
    pub(super) fn new(title: &'a str) -> Self {
        LineBreaker {
            title,
            lines: vec![],
            max_line_height: 0.,
            max_line_width: 0.,
        }
    }

    // TODO: support hyphenation
    pub(super) fn break_text(
        &mut self,
        width: f32,
        style: &Style,
        font: &Option<impl Font>,
        textarea: &TextArea,
        font_context: &FontContext,
    ) -> Result<(), Error> {
        let mut last_whitespace_idx = 0;
        // Space between whitespace and whitespace
        let mut word_width = 0.;
        let mut range = 0..0;
        let mut line_height = 0.;
        let mut line_width = 0.;
        let mut chars = RenderingCharIndices::from_str(self.title);
        while let Some((flags, i, ch, ch_len)) = chars.next() {
            let setting = match textarea.get_glyphs_from_char_range(i..i + ch_len) {
                (Some(split_text), _) => {
                    let style = split_text.style.as_ref().unwrap_or(style);
                    FontSetting {
                        size: style.font_size,
                        letter_spacing: style.letter_spacing,
                        kern_setting: style.kern_setting,
                        is_pre: style.white_space.is_pre(),
                    }
                }
                _ => FontSetting {
                    size: style.font_size,
                    letter_spacing: style.letter_spacing,
                    kern_setting: style.kern_setting,
                    is_pre: style.white_space.is_pre(),
                },
            };
            let whitespace_width = whitespace_width(setting.size);

            let peek_char = chars.peek_char();

            let extents = match font {
                Some(font) if match_font_family(ch, font) => textarea.char_extents(
                    ch,
                    peek_char,
                    &flags,
                    font,
                    i..i + ch_len,
                    font_context,
                    &setting,
                )?,
                _ => {
                    let idx = font_context.select_font_family('.')?;
                    font_context.with(&idx, |font| {
                        textarea.char_extents(
                            ch,
                            peek_char,
                            &flags,
                            font,
                            i..i + ch_len,
                            font_context,
                            &setting,
                        )
                    })?
                }
            };

            let ch_width = extents.width;
            let is_newline = matches!(flags, Some(CharFlags::Newline));

            if setting.is_pre && is_newline {
                let start = range.end + ch_len;
                self.lines.push(Line::new(
                    range.start..range.end + ch_len,
                    line_width,
                    line_height,
                ));
                self.set_max_line_size(FontMetrics {
                    height: line_height,
                    width: line_width,
                });
                range = start..start;
                line_width = 0.;
                line_height = 0.;
            }

            if width <= line_width + ch_width {
                match style.word_break {
                    WordBreak::Normal => {
                        let end = range.end;
                        line_width -= word_width + whitespace_width;
                        // TODO: support overflow text when text can not be broken.
                        self.lines.push(Line::new(
                            range.start..last_whitespace_idx,
                            line_width,
                            line_height,
                        ));
                        self.set_max_line_size(FontMetrics {
                            height: line_height,
                            width: line_width,
                        });
                        range = last_whitespace_idx..end;
                        line_width = word_width;
                        line_height = 0.;
                    }
                    WordBreak::BreakAll => {
                        let start = range.end;
                        self.lines.push(Line::new(range, line_width, line_height));
                        self.set_max_line_size(FontMetrics {
                            height: line_height,
                            width: line_width,
                        });
                        range = start..start;
                        line_width = 0.;
                        line_height = 0.;
                    }
                }
            }

            if setting.is_pre && is_newline {
                word_width = 0.;
            } else if ch.is_whitespace() {
                range.end = i + ch_len;
                line_width += whitespace_width;
                last_whitespace_idx = i + ch_len;
                word_width = 0.;
            } else {
                range.end = i + ch_len;
                line_width += ch_width;
                word_width += ch_width;
            }

            line_height = if extents.height > line_height {
                extents.height
            } else {
                line_height
            };
        }

        if !range.is_empty() {
            self.lines.push(Line::new(range, line_width, line_height));
            self.set_max_line_size(FontMetrics {
                height: line_height,
                width: line_width,
            });
        }

        Ok(())
    }

    // Calculate line size
    pub fn set_max_line_size(&mut self, metrics: FontMetrics) {
        let max_line_height = self.max_line_height;
        let max_line_width = self.max_line_width;

        self.max_line_height = if metrics.height > max_line_height {
            metrics.height
        } else {
            max_line_height
        };

        self.max_line_width = if metrics.width > max_line_width {
            metrics.width
        } else {
            max_line_width
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::test_utils::FontMock;
    use crate::font::FontContext;
    use crate::layout::TextArea;
    use crate::style::WhiteSpace;

    #[test]
    fn test_break_test_with_whitespace() {
        let width = 130u32;
        let _height = 50u32;

        let text = "Hello World, Hello World";
        let font_size = 10.;

        let font = FontMock;

        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font_context = FontContext::new();

        textarea
            .set_glyphs(&Some(font.clone()), &font_context)
            .unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                width as f32,
                &Style {
                    font_size,
                    word_break: WordBreak::Normal,
                    ..Style::default()
                },
                &Some(font),
                &textarea,
                &font_context,
            )
            .unwrap();

        let expects = ["Hello World, ", "Hello World"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.range.clone()] {
                panic!(
                    "expect '{}', but got '{}'",
                    expects[i],
                    &text[line.range.clone()]
                );
            }
        }
    }

    #[test]
    fn test_break_test_with_pre_line() {
        let width = 130u32;
        let _height = 50u32;

        let text = "Test\nHello World, Hello\nWorld";
        let font_size = 10.;
        let font = FontMock;

        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font_context = FontContext::new();

        textarea
            .set_glyphs(&Some(font.clone()), &font_context)
            .unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                width as f32,
                &Style {
                    font_size,
                    word_break: WordBreak::Normal,
                    white_space: WhiteSpace::PreLine,
                    ..Style::default()
                },
                &Some(font),
                &textarea,
                &font_context,
            )
            .unwrap();

        let expects = ["Test\n", "Hello World, ", "Hello\n", "World"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.range.clone()] {
                panic!(
                    "expect '{}', but got '{}'",
                    expects[i],
                    &text[line.range.clone()]
                );
            }
        }
    }

    #[test]
    fn test_break_with_newline_as_whitespace() {
        let width = 130u32;
        let _height = 50u32;

        let text = "Hello World,\nHello\nWorld";
        let font_size = 10.;
        let font = FontMock;

        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font_context = FontContext::new();

        textarea
            .set_glyphs(&Some(font.clone()), &font_context)
            .unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                width as f32,
                &Style {
                    font_size,
                    word_break: WordBreak::Normal,
                    white_space: WhiteSpace::Normal,
                    ..Style::default()
                },
                &Some(font),
                &textarea,
                &font_context,
            )
            .unwrap();

        let expects = ["Hello World,\n", "Hello\nWorld"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.range.clone()] {
                panic!(
                    "expect '{}', but got '{}'",
                    expects[i],
                    &text[line.range.clone()]
                );
            }
        }
    }

    #[test]
    fn test_break_with_split_text_newline() {
        let width = 130u32;
        let _height = 50u32;

        let text = "Test\nHello World, Hello\nWorld";
        let font_size = 10.;
        let font = FontMock;

        let mut textarea = TextArea::new();
        textarea
            .push(
                text,
                Style {
                    font_size,
                    white_space: WhiteSpace::PreLine,
                    ..Style::default()
                },
                None,
            )
            .unwrap();

        let font_context = FontContext::new();

        textarea
            .set_glyphs(&Some(font.clone()), &font_context)
            .unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                width as f32,
                &Style::default(),
                &Some(font),
                &textarea,
                &font_context,
            )
            .unwrap();

        let expects = ["Test\n", "Hello World, ", "Hello\n", "World"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.range.clone()] {
                panic!(
                    "expect '{}', but got '{}'",
                    expects[i],
                    &text[line.range.clone()]
                );
            }
        }
    }

    #[test]
    fn test_break_test_with_char() {
        let width = 90u32;
        let _height = 50u32;

        let text = "こんにちは世界、こんにちは世界";
        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font = FontMock;

        let font_context = FontContext::new();

        textarea
            .set_glyphs(&Some(font.clone()), &font_context)
            .unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                width as f32,
                &Style {
                    font_size: 10.,
                    word_break: WordBreak::BreakAll,
                    ..Style::default()
                },
                &Some(font),
                &textarea,
                &font_context,
            )
            .unwrap();

        let expects = ["こんにちは世界、", "こんにちは世界"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.range.clone()] {
                panic!(
                    "expect '{}', but got '{}'",
                    expects[i],
                    &text[line.range.clone()]
                );
            }
        }
    }
}
