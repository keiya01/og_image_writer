use super::textarea::TextArea;
use crate::element::{Element, Line, Rect, Text};
use crate::line_breaker::LineBreaker;
use crate::style::{FlexDirection, Margin, Position, Style, TextOverflow};
use crate::writer::OGImageWriter;
use crate::Error;
use rusttype::Font;
use std::cell::RefCell;
use std::str;

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

        let Margin(margin_top, margin_right, margin_bottom, margin_left) = style.margin;

        let (left, right) = if matches!(style.position, Position::Absolute) {
            (
                style.left.unwrap_or(0) + margin_left,
                style.right.unwrap_or(0) + margin_right,
            )
        } else {
            (margin_left, margin_right)
        };

        let text_area_width = match style.max_width {
            Some(max_width) => max_width as i32,
            None => window_width as i32,
        } - left
            - right;

        let text = textarea.borrow().as_string();

        let mut line_breaker = LineBreaker::new(&text);
        line_breaker.break_text(
            &self.context,
            text_area_width as f32,
            &style,
            &font,
            &textarea.borrow(),
        )?;

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

            if lines_len == 1 {
                total_height = next_height;
                lines.push(Line::new(
                    line.range,
                    Rect::new(0, 0, line.width as u32, line.height as u32),
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
                Rect::new(0, pos_y as u32, line.width as u32, line.height as u32),
            ));
        }

        let text = if is_overflow {
            self.set_ellipsis(
                &text[0..lines.last().unwrap().range.end],
                &mut lines,
                &style,
                &font,
                &mut textarea.borrow_mut(),
            )?
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
            max_line_width,
            textarea.into_inner(),
        )));

        // TODO: refactor
        if !text_elm.is_absolute() {
            match self.window.flex_direction {
                FlexDirection::Column => {
                    self.content.height +=
                        (total_height as i32 + margin_top + margin_bottom) as u32;
                    let next_width = (max_line_width as i32 + margin_left + margin_right) as u32;
                    self.content.width = if self.content.width > next_width {
                        self.content.width
                    } else {
                        next_width
                    };
                }
                FlexDirection::Row => {
                    self.content.width +=
                        (max_line_width as i32 + margin_left + margin_right) as u32;
                }
            }
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
    ) -> Result<String, Error> {
        let ellipsis = match style.text_overflow {
            TextOverflow::Ellipsis => "...",
            TextOverflow::Content(s) => s,
            TextOverflow::Clip => return Ok(text.to_string()),
        };

        let ellipsis_width = self
            .context
            .text_extents(ellipsis, style.font_size, font)
            .width;

        let mut total_char_width = 0.;
        let mut split_index = 0;
        for (i, ch) in text.char_indices().rev() {
            let split_text = textarea.get_split_text_from_char_range(i..i + ch.to_string().len());
            let (font_size, font) = match split_text {
                Some(split_text) => {
                    let font_size = match &split_text.style {
                        Some(style) => style.font_size,
                        None => style.font_size,
                    };
                    let font = match &split_text.font {
                        Some(font) => font,
                        None => font,
                    };
                    (font_size, font)
                }
                None => (style.font_size, font),
            };
            total_char_width += self.context.char_extents(ch, font_size, font).width;
            if total_char_width >= ellipsis_width {
                split_index = i;
                break;
            }
        }

        if let Some(line) = lines.last_mut() {
            // shape TextArea with ellipsis
            while let Some(mut split_text) = textarea.0.pop() {
                if split_text.range.start <= split_index && split_index <= split_text.range.end {
                    let end = split_text.range.end - split_index;
                    split_text.range.end -= end;
                    split_text.text = &split_text.text[0..split_text.text.len() - end];
                    textarea.0.push(split_text);
                    break;
                }
            }

            let next_range = line.range.start..split_index + ellipsis.len();
            line.range = next_range.clone();
            let mut next_text = text[0..split_index].to_string().clone();
            next_text.push_str(ellipsis);
            return Ok(next_text);
        }

        Ok(text.to_string())
    }
}
