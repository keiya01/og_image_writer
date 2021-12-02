use super::textarea::TextArea;
use crate::element::{Element, Line, LineMetrics, Rect, Text};
use crate::line_breaker::LineBreaker;
use crate::style::{FlexDirection, Margin, Position, Style, TextOverflow};
use crate::writer::OGImageWriter;
use crate::Error;
use rusttype::Font;
use std::cell::RefCell;
use std::str;

impl OGImageWriter {
    pub(crate) fn process_text(
        &mut self,
        textarea: RefCell<TextArea>,
        // Parent style that effect child element
        style: Style,
        // Parent font that effect child element
        font: Font<'static>,
    ) -> Result<(), Error> {
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

        textarea
            .borrow_mut()
            .set_glyphs(&font, &self.font_context)?;

        let mut line_breaker = LineBreaker::new(&text);
        line_breaker.break_text(
            &self.context,
            text_area_width as f32,
            &style,
            &font,
            &textarea.borrow(),
            &self.font_context,
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
            LineMetrics::new(total_height as u32, max_line_height, max_line_width),
            style,
            font,
            textarea.into_inner(),
        )));

        // TODO: refactor
        if !text_elm.is_absolute() {
            match self.window.flex_direction {
                FlexDirection::Column => {
                    self.content.height +=
                        (total_height as i32 + margin_top + margin_bottom) as u32;
                }
                FlexDirection::Row => {
                    self.content.width +=
                        (max_line_width as i32 + margin_left + margin_right) as u32;
                }
            }
        }

        self.tree.0.push(text_elm);

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
        let ellipsis = match &style.text_overflow {
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
            let extents = textarea.char_extents(
                ch,
                font,
                i..i + ch.to_string().len(),
                style,
                &self.context,
                &self.font_context,
            )?;
            total_char_width += extents.width;
            if total_char_width >= ellipsis_width {
                split_index = i;
                break;
            }
        }

        if let Some(line) = lines.last_mut() {
            // shape TextArea with ellipsis
            while let Some(mut split_text) = textarea.0.pop() {
                if split_text.range.start <= split_index && split_index <= split_text.range.end {
                    while let Some(mut glyph) = split_text.glyphs.pop() {
                        if glyph.range.start <= split_index && split_index <= glyph.range.end {
                            let end = glyph.range.end - split_index;
                            glyph.range.end -= end;
                            split_text.glyphs.push(glyph);
                            break;
                        }
                    }

                    let end = split_text.range.end - split_index;
                    split_text.range.end -= end;
                    split_text.text =
                        (&split_text.text[0..split_text.text.len() - end]).to_string();
                    textarea.0.push(split_text);
                    break;
                }
            }

            line.range = line.range.start..split_index + ellipsis.len();
            let mut next_text = text[0..split_index].to_string().clone();
            next_text.push_str(ellipsis);
            textarea.push_text_with_glyphs(ellipsis, font, &self.font_context)?;

            return Ok(next_text);
        }

        Ok(text.to_string())
    }
}
