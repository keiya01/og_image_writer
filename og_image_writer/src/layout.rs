use super::element::{Element, Img, Line, Rect, Text};
use super::image::open_and_resize;
use super::line_breaker::LineBreaker;
use super::style::{AlignItems, JustifyContent, Margin, Style, TextAlign, WordBreak, TextOverflow};
use super::writer::{set_font, OGImageWriter};
use std::{io, str};

impl<'a> OGImageWriter<'a> {
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
        let Margin(margin_top, _, margin_bottom, _) = style.margin;

        let (bytes, size) = open_and_resize(src, width, height)?;

        let logical_block = match &self.window.justify_content {
            JustifyContent::Start | JustifyContent::Center => margin_top,
            JustifyContent::End => -margin_bottom,
        };

        let img = Element::Img(Some(Img::new(
            bytes,
            size.width,
            size.height,
            Rect::new(0., logical_block),
            style,
        )));

        self.content.height += size.height;
        self.content.width = if self.content.width > size.width {
            self.content.width
        } else {
            size.width
        };

        self.tree.push(img);

        Ok(())
    }

    pub(super) fn process_text(&mut self, text: &'a str, style: Style<'a>) {
        let window_width = self.window.width as f64;

        let Margin(margin_top, margin_left, margin_bottom, margin_right) = style.margin;

        let text_area_width = window_width - margin_left - margin_right;

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
              _ => {},
            }

            set_font(&self.context, &style);

            let text = &text[line.clone()];

            let extents = self.context.text_extents(text).unwrap();

            max_line_width = if extents.x_advance > max_line_width {
                extents.x_advance
            } else {
                max_line_width
            };

            let logical_block = match &self.window.justify_content {
                JustifyContent::Start | JustifyContent::Center => max_line_height + margin_top,
                JustifyContent::End => -margin_bottom,
            };

            if lines_len == 1 {
                total_height = next_height;
                lines.push(Line::new(line, Rect::new(0., logical_block)));
                break;
            }

            let pos_y = logical_block + total_height;
            let pos_y = if !is_first_line {
                pos_y + line_height
            } else {
                pos_y
            };

            total_height = next_height;
            lines.push(Line::new(line, Rect::new(0., pos_y)));
        }

        let text = if is_overflow {
          self.set_ellipsis(&text[0..lines.last().unwrap().range.end], &mut lines, &style)
        } else {
          text.to_string()
        };

        self.content.height += total_height as u32;
        self.content.width = if self.content.width > max_line_width as u32 {
            self.content.width
        } else {
            max_line_width as u32
        };

        self.tree.push(Element::Text(Some(Text::new(
            text,
            lines,
            total_height,
            style,
        ))));
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
        total_char_width += self.context.text_extents(&ch.to_string()).unwrap().x_advance;
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

    pub(super) fn process_flexbox(&mut self) {
        let total_height = if self.content.height as i32 > self.window.height {
            self.window.height as f64
        } else {
            self.content.height as f64
        };
        let content_width = if self.content.width as i32 > self.window.width {
            self.window.width as f64
        } else {
            self.content.width as f64
        };

        let center_height = total_height / 2.;
        let rest_height = self.window.height as f64 / 2. - center_height;

        let window_height = self.window.height as f64;
        let window_width = self.window.width as f64;

        let logical_block = match &self.window.justify_content {
            JustifyContent::Start => 0.,
            JustifyContent::Center => rest_height,
            JustifyContent::End => window_height,
        };

        let is_end = if let JustifyContent::End = self.window.justify_content {
          self.tree.reverse();
          true
        } else {
          false
        };

        let mut current_y = logical_block;

        for elm in &mut self.tree {
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
                      img.rect.y += current_y - img.height as f64;
                      current_y -= img.height as f64 + margin_top;
                    } else {
                      img.rect.y += current_y;
                      current_y += img.height as f64 + margin_bottom;
                    }
                }
                Element::Text(Some(text)) => {
                    let Margin(margin_top, margin_left, margin_bottom, margin_right) = text.style.margin;

                    for line in &mut text.lines {
                        let logical_inline = match &self.window.align_items {
                            AlignItems::Start => margin_left,
                            AlignItems::Center => window_width / 2. - content_width / 2.,
                            AlignItems::End => window_width - content_width - margin_right,
                        };

                        let text_content = &text.text[line.range.clone()];
                        let extents = self.context.text_extents(text_content).unwrap();
                        let content_box_inline = match text.style.text_align {
                            TextAlign::Start => 0.,
                            TextAlign::Center => content_width / 2. - extents.x_advance / 2.,
                            TextAlign::End => content_width - extents.x_advance,
                        } + logical_inline;

                        line.rect.x = content_box_inline;
                        if is_end {
                          line.rect.y += current_y - text.total_height;
                        } else {
                          line.rect.y += current_y;
                        }
                    }

                    if is_end {
                      current_y -= text.total_height + margin_top;
                    } else {
                      current_y += text.total_height + margin_bottom;
                    }
                }
                _ => {}
            }
        }
    }
}
