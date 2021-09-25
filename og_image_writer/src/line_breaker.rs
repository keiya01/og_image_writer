use super::context::Context;
use rusttype::Font;
use std::ops::Range;

pub struct LineBreaker<'a> {
    pub(super) title: &'a str,
    pub(super) lines: Vec<Range<usize>>,
}

// TODO: support truncate text when overflow specified height.
impl<'a> LineBreaker<'a> {
    pub(super) fn new(title: &'a str) -> Self {
        LineBreaker {
            title,
            lines: vec![],
        }
    }

    // TODO: support hyphenation
    pub(super) fn break_text_with_whitespace(
        &mut self,
        context: &Context,
        width: f32,
        font_size: f32,
        font: &Font,
    ) {
        let text_arr: Vec<&str> = self.title.split_whitespace().collect();

        let text_arr_len = text_arr.len();

        let whitespace_width = context.text_extents(" ", font_size, font).width;
        let whitespace_idx = 1;

        let mut line = 0..0;
        let mut line_width = 0.;
        for (i, text) in text_arr.into_iter().enumerate() {
            let extents = context.text_extents(text, font_size, font);

            let is_last = text_arr_len - 1 == i;

            let text_width = extents.width;
            let text_width = if is_last {
                text_width
            } else {
                text_width + whitespace_width
            };

            if width <= line_width + text_width {
                let start = line.end;
                self.lines.push(line);
                line = start..start;
                line_width = 0.;
            }

            line.end += text.len() + whitespace_idx;
            line_width += text_width;
        }

        // End of line should not have whitespace
        line.end -= whitespace_idx;

        self.lines.push(line);
    }

    pub(super) fn break_text_with_char(
        &mut self,
        context: &Context,
        width: f32,
        font_size: f32,
        font: &Font,
    ) {
        let chars = self.title.char_indices();

        let mut line = 0..0;
        let mut line_width = 0.;
        for (i, ch) in chars.into_iter() {
            let extents = context.text_extents(&ch.to_string(), font_size, font);

            let ch_width = extents.width;

            if width <= line_width + ch_width {
                let start = line.end;
                self.lines.push(line);
                line = start..start;
                line_width = 0.;
            }

            line.end = i + ch.to_string().len();
            line_width += ch_width;
        }

        self.lines.push(line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Context;
    use rusttype::Font;

    #[test]
    fn test_break_test_with_whitespace() {
        let width = 80u32;
        let height = 50u32;
        let context = Context::new(width, height);

        let text = "Hello World, Hello World";
        let mut line_breaker = LineBreaker::new(text);
        line_breaker.break_text_with_whitespace(
            &context,
            width as f32,
            16.,
            &Font::try_from_bytes(include_bytes!("../../fonts/Mplus1-Black.ttf")).unwrap(),
        );

        let expects = ["Hello World, ", "Hello World"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.clone()] {
                panic!("expect '{}', but got '{}'", expects[i], &text[line.clone()]);
            }
        }
    }

    #[test]
    fn test_break_test_with_char() {
        let width = 90u32;
        let height = 50u32;
        let context = Context::new(width, height);

        let text = "こんにちは世界、こんにちは世界";
        let mut line_breaker = LineBreaker::new(text);
        line_breaker.break_text_with_char(
            &context,
            width as f32,
            16.,
            &Font::try_from_bytes(include_bytes!("../../fonts/Mplus1-Black.ttf")).unwrap(),
        );

        let expects = ["こんにちは世界、", "こんにちは世界"];

        for (i, line) in line_breaker.lines.iter().enumerate() {
            if expects[i] != &text[line.clone()] {
                panic!("expect '{}', but got '{}'", expects[i], &text[line.clone()]);
            }
        }
    }
}
