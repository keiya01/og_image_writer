use super::element::{Element, Img, Line, Rect, Text};
use super::img::{open_and_resize, open_and_resize_with_data, round};
use super::line_breaker::LineBreaker;
use super::style::{
    AlignItems, JustifyContent, Margin, Position, Style, TextAlign, TextOverflow, WordBreak,
};
use super::writer::OGImageWriter;
use rusttype::Font;
use std::str;

impl<'a> OGImageWriter<'a> {
    pub(super) fn process(&mut self) {
        let total_height = if self.content.height > self.window.height {
            self.window.height
        } else {
            self.content.height
        };

        let center_height = total_height / 2;
        let rest_height = self.window.height / 2 - center_height;

        let window_height = self.window.height;

        let logical_block = match &self.window.justify_content {
            JustifyContent::Start => 0,
            JustifyContent::Center => rest_height,
            JustifyContent::End => window_height,
        };

        if !matches!(self.window.justify_content, JustifyContent::End) {
            self.tree.reverse();
        }

        let mut current_y = logical_block as i32;

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

    fn process_flexbox(&mut self, elm: &mut Element, current_y: &mut i32) {
        let window_width = self.window.width as i32;
        let content_width = if self.content.width > self.window.width {
            window_width
        } else {
            self.content.width as i32
        };
        let is_end = matches!(self.window.justify_content, JustifyContent::End);
        match elm {
            Element::Img(Some(img)) => {
                let Margin(margin_top, margin_left, margin_bottom, margin_right) = img.style.margin;

                let logical_inline = match &self.window.align_items {
                    AlignItems::Start => margin_left,
                    AlignItems::Center => window_width / 2 - content_width / 2,
                    AlignItems::End => window_width - content_width - margin_right,
                };

                let content_box_inline = match img.style.text_align {
                    TextAlign::Start => 0,
                    TextAlign::Center => content_width / 2 - (img.width as i32 / 2),
                    TextAlign::End => content_width - img.width as i32,
                } + logical_inline;

                img.rect.x = content_box_inline as u32;

                if is_end {
                    img.rect.y += (*current_y - img.height as i32) as u32;
                    *current_y -= img.height as i32 + margin_top;
                } else {
                    img.rect.y += *current_y as u32;
                    *current_y += img.height as i32 + margin_bottom;
                }
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_left, margin_bottom, margin_right) =
                    text.style.margin;

                for line in &mut text.lines {
                    let logical_inline = match &self.window.align_items {
                        AlignItems::Start => margin_left,
                        AlignItems::Center => 0,
                        AlignItems::End => -margin_right,
                    };

                    // Because imageproc draw text that include line_height.
                    let system_line_height = text.max_line_height as u32 / 2;

                    line.rect.x += logical_inline as u32;
                    if is_end {
                        line.rect.y += (*current_y - text.total_height as i32 - margin_bottom)
                            as u32
                            - system_line_height;
                    } else {
                        line.rect.y += (*current_y + margin_top) as u32;
                    }

                    if matches!(self.window.justify_content, JustifyContent::Center) {
                        line.rect.y -= system_line_height;
                    }
                }

                if is_end {
                    *current_y -= text.total_height as i32 + margin_top;
                } else {
                    *current_y += text.total_height as i32 + margin_bottom;
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
                    (Some(left), _) => left as i32 + margin_left,
                    (None, Some(right)) => {
                        self.window.width as i32 - img.width as i32 - right - margin_right
                    }
                    (None, None) => margin_left,
                } as u32;
                img.rect.y += match (img.style.top, img.style.bottom) {
                    (Some(top), _) => top + margin_top,
                    (None, Some(bottom)) => {
                        self.window.height as i32 - img.height as i32 - bottom - margin_bottom
                    }
                    (None, None) => margin_top,
                } as u32;
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) =
                    text.style.margin;

                for line in &mut text.lines {
                    line.rect.x += match (text.style.left, text.style.right) {
                        (Some(left), _) => left + margin_left,
                        (None, Some(right)) => -(right - margin_right),
                        (None, None) => margin_left,
                    } as u32;
                    line.rect.y += match (text.style.top, text.style.bottom) {
                        (Some(top), _) => top + margin_top,
                        (None, Some(bottom)) => {
                            self.window.height as i32
                                - text.total_height as i32
                                - bottom
                                - margin_bottom
                        }
                        (None, None) => margin_top,
                    } as u32;
                }
            }
            _ => {}
        }
    }

    pub(super) fn process_background(&mut self) {
        let window = &self.window;
        let background_color = match &window.background_color {
            None => return,
            Some(color) => color,
        };

        self.context.draw_background_color(*background_color);
    }

    fn process_img(
        &mut self,
        img: Element<'a>,
        width: u32,
        height: u32,
    ) {
        if !img.is_absolute() {
            self.content.height += height;
            self.content.width = if self.content.width > width {
                self.content.width
            } else {
                width
            };
        }

        self.tree.push(img);
    }

    pub(super) fn process_img_with_src(
        &mut self,
        src: &'a str,
        width: u32,
        height: u32,
        style: Style<'a>,
    ) {
        let (mut buf, size) = open_and_resize(src, width, height);

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone(), 0.);

        let img = Element::Img(Some(Img::new(
            buf,
            size.width,
            size.height,
            Rect::new(0, 0),
            style,
        )));

        self.process_img(img, size.width, size.height);
    }

    pub(super) fn process_img_with_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        style: Style<'a>,
    ) {
        let (mut buf, size) = open_and_resize_with_data(data, width, height);

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone(), 0.);

        let img = Element::Img(Some(Img::new(
            buf,
            size.width,
            size.height,
            Rect::new(0, 0),
            style,
        )));

        self.process_img(img, size.width, size.height);
    }

    pub(super) fn process_text(&mut self, text: &'a str, style: Style<'a>, font: Vec<u8>) {
        let font = Font::try_from_vec(font).expect("Could not parse font data");

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

        let mut line_breaker = LineBreaker::new(text);
        match style.word_break {
            WordBreak::Normal => line_breaker.break_text_with_whitespace(
                &self.context,
                text_area_width as f32,
                style.font_size,
                &font,
            ),
            WordBreak::BreakAll => line_breaker.break_text_with_char(
                &self.context,
                text_area_width as f32,
                style.font_size,
                &font,
            ),
        }

        let mut max_line_height = 0.;
        let mut max_line_width = 0.;
        for line in &line_breaker.lines {
            let extents = self
                .context
                .text_extents(&text[line.clone()], style.font_size, &font);

            max_line_height = if extents.height > max_line_height {
                extents.height
            } else {
                max_line_height
            };

            max_line_width = if extents.width > max_line_width {
                extents.width
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

            let text_content = &text[line.clone()];

            let extents = self
                .context
                .text_extents(text_content, style.font_size, &font);
            let content_box_inline = match style.text_align {
                TextAlign::Start => 0.,
                TextAlign::Center => max_line_width / 2. - extents.width / 2.,
                TextAlign::End => max_line_width - extents.width,
            } + logical_inline;

            if lines_len == 1 {
                total_height = next_height;
                lines.push(Line::new(line, Rect::new(content_box_inline as u32, 0)));
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
                line,
                Rect::new(content_box_inline as u32, pos_y as u32),
            ));
        }

        let text = if is_overflow {
            self.set_ellipsis(
                &text[0..lines.last().unwrap().range.end],
                &mut lines,
                &style,
                &font,
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

    fn set_ellipsis(
        &mut self,
        text: &str,
        lines: &mut Vec<Line>,
        style: &Style,
        font: &Font,
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
            let next_range = line.range.start..split_index + ellipsis.len();
            line.range = next_range.clone();
            let mut next_text = text[0..split_index].to_string().clone();
            next_text.push_str(ellipsis);
            return next_text;
        }

        text.to_string()
    }
}
