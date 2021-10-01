use super::textarea::TextArea;
use crate::element::{Element, Line, Rect, Text};
use crate::line_breaker::LineBreaker;
use crate::style::{AlignItems, Margin, Position, Style, TextAlign, TextOverflow};
use crate::writer::OGImageWriter;
use crate::Error;
use rusttype::Font;
use std::str;
use std::cell::RefCell;

impl<'a> OGImageWriter<'a> {
    pub(crate) fn process_text(
        &mut self,
        textarea: RefCell<TextArea<'a>>,
        // Parent style that effect child element
        style: Style<'a>,
        // Parent font that effect child element
        font: Vec<u8>,
    ) -> Result<(), Error> {
        let font = match Font::try_from_vec(font) {
            Some(font) => font,
            None => return Err(Error::InvalidFontBytes),
        };

        let window_width = self.window.width as f32;

        let Margin(_, margin_left, _, margin_right) = style.margin;

        let (left, right) = if matches!(style.position, Position::Absolute) {
            (
                style.left.unwrap_or(0) + margin_left,
                style.right.unwrap_or(0) + margin_right,
            )
        } else {
            (margin_left, margin_right)
        };

        let text_area_width = window_width as i32 - left - right;

        let text = textarea.borrow().as_string();

        let mut line_breaker = LineBreaker::new(&text);
        line_breaker.break_text(
            &self.context,
            text_area_width as f32,
            &style,
            &font,
            &textarea.borrow(),
        )?;

        // Calculate line size
        let max_line_height = line_breaker.max_line_height;
        let max_line_width = line_breaker.max_line_width;

        let mut lines: Vec<Line> = vec![];

        // Calculate line position
        let mut total_height = 0.;
        let line_height = max_line_height * style.line_height / 2. - max_line_height / 2.;
        let lines_len = line_breaker.lines.len();
        let mut is_overflow = false;
        for (i, line) in line_breaker.lines.into_iter().enumerate() {
            let is_first_line = i == 0;
            let next_height = if is_first_line {
                total_height + max_line_height
            } else {
                total_height + max_line_height + line_height
            };

            match style.max_height {
                Some(max_height) if next_height > max_height as f32 => {
                    is_overflow = true;
                    break;
                }
                _ => {}
            }

            let logical_inline = match &self.window.align_items {
                AlignItems::Start => 0.,
                AlignItems::Center => window_width / 2. - max_line_width / 2.,
                AlignItems::End => window_width - max_line_width,
            };

            let content_box_inline = match style.text_align {
                TextAlign::Start => 0.,
                TextAlign::Center => max_line_width / 2. - line.width / 2.,
                TextAlign::End => max_line_width - line.width,
            } + logical_inline;

            if lines_len == 1 {
                total_height = next_height;
                lines.push(Line::new(
                    line.range,
                    Rect::new(content_box_inline as u32, 0),
                ));
                break;
            }

            let pos_y = total_height;
            let pos_y = if !is_first_line {
                pos_y + line_height
            } else {
                pos_y
            };

            total_height = next_height;
            lines.push(Line::new(
                line.range,
                Rect::new(content_box_inline as u32, pos_y as u32),
            ));
        }

        let text =
        if is_overflow {
            self.set_ellipsis(
                &text[0..lines.last().unwrap().range.end],
                &mut lines,
                &style,
                &font,
                &mut textarea.borrow_mut(),
            )
        } else {
            text.to_string()
        };

        let text_elm = Element::Text(Some(Text::new(
            text,
            lines,
            total_height as u32,
            style,
            font,
            max_line_height,
            textarea.into_inner(),
        )));

        if !text_elm.is_absolute() {
            self.content.height += total_height as u32;
            self.content.width = if self.content.width > max_line_width as u32 {
                self.content.width
            } else {
                max_line_width as u32
            };
        }

        self.tree.push(text_elm);

        Ok(())
    }

    fn set_ellipsis(
        &mut self,
        text: &str,
        lines: &mut Vec<Line>,
        style: &Style,
        font: &Font,
        textarea: &mut TextArea,
    ) -> String {
        let ellipsis = match style.text_overflow {
            TextOverflow::Ellipsis => "...",
            TextOverflow::Content(s) => s,
            TextOverflow::Clip => return text.to_string(),
        };

        let ellipsis_width = self
            .context
            .text_extents(ellipsis, style.font_size, font)
            .width;

        let mut total_char_width = 0.;
        let mut split_index = 0;
        for (i, ch) in text.char_indices().rev() {
            total_char_width += self
                .context
                .text_extents(&ch.to_string(), style.font_size, font)
                .width;
            if total_char_width > ellipsis_width {
                break;
            }
            split_index = i;
        }

        if let Some(line) = lines.last_mut() {
            // shape TextArea with ellipsis
            let mut total_ellipsis_len = ellipsis.len();
            while let Some(mut split_text) = textarea.0.pop() {
                if split_text.range.len() > total_ellipsis_len {
                    split_text.range.end -= total_ellipsis_len;
                    textarea.0.push(split_text);
                    break;
                } else {
                    total_ellipsis_len -= split_text.range.len();
                }
            }

            let next_range = line.range.start..split_index + ellipsis.len();
            line.range = next_range.clone();
            let mut next_text = text[0..split_index].to_string().clone();
            next_text.push_str(ellipsis);
            return next_text;
        }

        text.to_string()
    }
}
