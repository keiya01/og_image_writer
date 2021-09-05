use super::element::{Element, Img, Line, Rect, Text};
use super::image::open_and_resize;
use super::line_breaker::LineBreaker;
use super::style::{
    AlignItems, JustifyContent, Margin, Position, Style, TextAlign, TextOverflow, WordBreak,
};
use super::writer::{set_font, OGImageWriter};
use std::{io, str};

impl<'a> OGImageWriter<'a> {
    pub(super) fn process(&mut self) {
        let total_height = if self.content.height as i32 > self.window.height {
            self.window.height as f64
        } else {
            self.content.height as f64
        };

        let center_height = total_height / 2.;
        let rest_height = self.window.height as f64 / 2. - center_height;

        let window_height = self.window.height as f64;

        let logical_block = match &self.window.justify_content {
            JustifyContent::Start => 0.,
            JustifyContent::Center => rest_height,
            JustifyContent::End => window_height,
        };

        if !matches!(self.window.justify_content, JustifyContent::End) {
            self.tree.reverse();
        }

        let mut current_y = logical_block;

        let mut tree = OGImageWriter::create_tree();
        while let Some(mut elm) = self.tree.pop() {
            if elm.is_absolute() {
                self.process_absolute(&mut elm);
            } else {
                self.process_flexbox(&mut elm, &mut current_y);
            }
            tree.push(elm);
        }
        self.tree.append(&mut tree);
    }

    fn process_flexbox(&mut self, elm: &mut Element, current_y: &mut f64) {
        let window_width = self.window.width as f64;
        let content_width = if self.content.width as i32 > self.window.width {
            self.window.width as f64
        } else {
            self.content.width as f64
        };
        let is_end = matches!(self.window.justify_content, JustifyContent::End);
        match elm {
            Element::Img(Some(img)) => {
                let Margin(margin_top, margin_left, margin_bottom, margin_right) = img.style.margin;

                let logical_inline = match &self.window.align_items {
                    AlignItems::Start => margin_left,
                    AlignItems::Center => window_width / 2. - content_width / 2.,
                    AlignItems::End => window_width - content_width - margin_right,
                };

                let content_box_inline = match img.style.text_align {
                    TextAlign::Start => 0.,
                    TextAlign::Center => content_width / 2. - (img.width / 2) as f64,
                    TextAlign::End => content_width - img.width as f64,
                } + logical_inline;

                img.rect.x = content_box_inline;

                if is_end {
                    img.rect.y += *current_y - img.height as f64;
                    *current_y -= img.height as f64 + margin_top;
                } else {
                    img.rect.y += *current_y;
                    *current_y += img.height as f64 + margin_bottom;
                }
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_left, margin_bottom, margin_right) =
                    text.style.margin;

                for line in &mut text.lines {
                    let logical_inline = match &self.window.align_items {
                        AlignItems::Start => margin_left,
                        AlignItems::Center => 0.,
                        AlignItems::End => -margin_right,
                    };

                    line.rect.x += logical_inline;
                    if is_end {
                        line.rect.y += *current_y - text.total_height - margin_bottom;
                    } else {
                        line.rect.y += *current_y + margin_top;
                    }
                }

                if is_end {
                    *current_y -= text.total_height + margin_top;
                } else {
                    *current_y += text.total_height + margin_bottom;
                }
            }
            _ => {}
        }
    }

    fn process_absolute(&mut self, elm: &mut Element) {
        match elm {
            Element::Img(Some(img)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) = img.style.margin;

                img.rect.x += match (img.style.left, img.style.right) {
                    (Some(left), _) => left + margin_left,
                    (None, Some(right)) => {
                        self.window.width as f64 - img.width as f64 - right - margin_right
                    }
                    (None, None) => margin_left,
                };
                img.rect.y += match (img.style.top, img.style.bottom) {
                    (Some(top), _) => top + margin_top,
                    (None, Some(bottom)) => {
                        self.window.height as f64 - img.height as f64 - bottom - margin_bottom
                    }
                    (None, None) => margin_top,
                };
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) =
                    text.style.margin;

                for line in &mut text.lines {
                    line.rect.x += match (text.style.left, text.style.right) {
                        (Some(left), _) => left + margin_left,
                        (None, Some(right)) => -(right - margin_right),
                        (None, None) => margin_left,
                    };
                    line.rect.y += match (text.style.top, text.style.bottom) {
                        (Some(top), _) => top + margin_top,
                        (None, Some(bottom)) => {
                            self.window.height as f64
                                - text.total_height as f64
                                - bottom
                                - margin_bottom
                        }
                        (None, None) => margin_top,
                    };
                }
            }
            _ => {}
        }
    }

    pub(super) fn process_background(&self) {
        let window = &self.window;
        let background_color = match &window.background_color {
            None => return,
            Some(color) => color,
        };

        self.context
            .set_source_rgb(background_color.0, background_color.1, background_color.2);
        self.context
            .paint()
            .expect("Could not paint specified background_color");
    }

    pub(super) fn process_img(
        &mut self,
        src: &'a str,
        width: u32,
        height: u32,
        style: Style<'a>,
    ) -> io::Result<()> {
        let (bytes, size) = open_and_resize(src, width, height)?;

        let img = Element::Img(Some(Img::new(
            bytes,
            size.width,
            size.height,
            Rect::new(0., 0.),
            style,
        )));

        if !img.is_absolute() {
            self.content.height += size.height;
            self.content.width = if self.content.width > size.width {
                self.content.width
            } else {
                size.width
            };
        }

        self.tree.push(img);

        Ok(())
    }

    pub(super) fn process_text(&mut self, text: &'a str, style: Style<'a>) {
        let window_width = self.window.width as f64;

        let Margin(_, margin_left, _, margin_right) = style.margin;

        let (left, right) = if matches!(style.position, Position::Absolute) {
            (
                style.left.unwrap_or(0.) + margin_left,
                style.right.unwrap_or(0.) + margin_right,
            )
        } else {
            (margin_left, margin_right)
        };

        let text_area_width = window_width - left - right;

        // Initialize font metrics for line breaking.
        set_font(&self.context, &style);

        let mut line_breaker = LineBreaker::new(text);
        match style.word_break {
            WordBreak::Normal => {
                line_breaker.break_text_with_whitespace(&self.context, text_area_width)
            }
            WordBreak::BreakAll => {
                line_breaker.break_text_with_char(&self.context, text_area_width)
            }
        }

        let mut max_line_height = 0.;
        let mut max_line_width = 0.;
        for line in &line_breaker.lines {
            let extents = self.context.text_extents(&text[line.clone()]).unwrap();

            max_line_height = if extents.height > max_line_height {
                extents.height
            } else {
                max_line_height
            };

            max_line_width = if extents.x_advance > max_line_width {
                extents.x_advance
            } else {
                max_line_width
            };
        }

        let mut lines: Vec<Line> = vec![];

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
                Some(max_height) if next_height > max_height => {
                    is_overflow = true;
                    break;
                }
                _ => {}
            }

            set_font(&self.context, &style);

            let logical_inline = match &self.window.align_items {
                AlignItems::Start => 0.,
                AlignItems::Center => window_width / 2. - max_line_width / 2.,
                AlignItems::End => window_width - max_line_width,
            };

            let text_content = &text[line.clone()];

            let extents = self.context.text_extents(text_content).unwrap();
            let content_box_inline = match style.text_align {
                TextAlign::Start => 0.,
                TextAlign::Center => max_line_width / 2. - extents.x_advance / 2.,
                TextAlign::End => max_line_width - extents.x_advance,
            } + logical_inline;

            if lines_len == 1 {
                total_height = next_height;
                lines.push(Line::new(
                    line,
                    Rect::new(content_box_inline, 0.),
                ));
                break;
            }

            let pos_y = max_line_height + total_height;
            let pos_y = if !is_first_line {
                pos_y + line_height
            } else {
                pos_y
            };

            total_height = next_height;
            lines.push(Line::new(line, Rect::new(content_box_inline, pos_y)));
        }

        let text = if is_overflow {
            self.set_ellipsis(
                &text[0..lines.last().unwrap().range.end],
                &mut lines,
                &style,
            )
        } else {
            text.to_string()
        };

        let text_elm = Element::Text(Some(Text::new(
            text,
            lines,
            total_height,
            style,
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
    }

    fn set_ellipsis(&mut self, text: &str, lines: &mut Vec<Line>, style: &Style) -> String {
        let ellipsis = match style.text_overflow {
            TextOverflow::Ellipsis => "...",
            TextOverflow::Content(s) => s,
            TextOverflow::Clip => return text.to_string(),
        };

        let ellipsis_width = self.context.text_extents(ellipsis).unwrap().x_advance;

        let mut total_char_width = 0.;
        let mut split_index = 0;
        for (i, ch) in text.char_indices().rev() {
            total_char_width += self
                .context
                .text_extents(&ch.to_string())
                .unwrap()
                .x_advance;
            if total_char_width > ellipsis_width {
                break;
            }
            split_index = i;
        }

        if let Some(line) = lines.last_mut() {
            let next_range = line.range.start..split_index + ellipsis.len();
            line.range = next_range.clone();
            let mut next_text = text[0..split_index].to_string().clone();
            next_text.push_str(ellipsis);
            return next_text;
        }

        text.to_string()
    }
}
