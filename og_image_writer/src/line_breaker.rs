use super::context::{Context, FontMetrics};
use super::layout::TextArea;
use crate::font::FontContext;
use crate::style::{Style, WordBreak};
use crate::Error;
use rusttype::Font;
use std::ops::Range;

pub(super) struct Line {
    pub(super) range: Range<usize>,
    pub(super) width: f32,
    pub(super) height: f32,
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
        context: &Context,
        width: f32,
        style: &Style,
        font: &Font,
        textarea: &TextArea,
        font_context: &FontContext,
    ) -> Result<(), Error> {
        let chars = self.title.char_indices();

        let mut last_whitespace_idx = 0;
        let mut last_whitespace_width = 0.;
        // Space between whitespace and whitespace
        let mut word_width = 0.;
        let mut range = 0..0;
        let mut line_height = 0.;
        let mut line_width = 0.;
        for (i, ch) in chars.into_iter() {
            let ch_len = ch.to_string().len();
            let extents =
                textarea.char_extents(ch, font, i..i + ch_len, style, context, font_context)?;

            let ch_width = extents.width;

            if width <= line_width + ch_width {
                match style.word_break {
                    WordBreak::Normal => {
                        let end = range.end;
                        line_width -= word_width + last_whitespace_width;
                        self.lines.push(Line {
                            range: range.start..last_whitespace_idx,
                            height: line_height,
                            width: line_width,
                        });
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
                        self.lines.push(Line {
                            range,
                            height: line_height,
                            width: line_width,
                        });
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

            range.end = i + ch_len;
            line_width += ch_width;
            word_width += ch_width;
            if ch.is_whitespace() {
                last_whitespace_idx = i + ch_len;
                last_whitespace_width = extents.width;
                word_width = 0.;
            }
            line_height = if extents.height > line_height {
                extents.height
            } else {
                line_height
            };
        }

        self.lines.push(Line {
            range,
            height: line_height,
            width: line_width,
        });
        self.set_max_line_size(FontMetrics {
            height: line_height,
            width: line_width,
        });

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
    use crate::context::Context;
    use crate::font::FontContext;
    use crate::layout::TextArea;
    use rusttype::Font;

    #[test]
    fn test_break_test_with_whitespace() {
        let width = 80u32;
        let height = 50u32;
        let context = Context::new(width, height);

        let text = "Hello World, Hello World";
        let font_size = 16.;
        let font = Font::try_from_bytes(include_bytes!("../../fonts/Mplus1-Black.ttf")).unwrap();

        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font_context = FontContext::new();

        textarea.set_glyphs(&font, &font_context).unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                &context,
                width as f32,
                &Style {
                    font_size,
                    word_break: WordBreak::Normal,
                    ..Style::default()
                },
                &font,
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
    fn test_break_test_with_char() {
        let width = 90u32;
        let height = 50u32;
        let context = Context::new(width, height);

        let text = "こんにちは世界、こんにちは世界";
        let mut textarea = TextArea::new();
        textarea.push_text(text);

        let font = Font::try_from_bytes(include_bytes!("../../fonts/Mplus1-Black.ttf")).unwrap();

        let font_context = FontContext::new();

        textarea.set_glyphs(&font, &font_context).unwrap();

        let mut line_breaker = LineBreaker::new(text);
        line_breaker
            .break_text(
                &context,
                width as f32,
                &Style {
                    font_size: 16.,
                    word_break: WordBreak::BreakAll,
                    ..Style::default()
                },
                &font,
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
