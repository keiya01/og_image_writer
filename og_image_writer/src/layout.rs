mod container;
mod img;
mod text;

use super::element::Element;
use super::style::{AlignItems, JustifyContent, Margin, TextAlign};
use super::writer::OGImageWriter;
use super::Error;

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

                // Because imageproc draw text that include line_height.
                let mut system_line_height = text.max_line_height as u32 / 2;

                for line in &mut text.lines {
                    let logical_inline = match &self.window.align_items {
                        AlignItems::Start => margin_left,
                        AlignItems::Center => 0,
                        AlignItems::End => -margin_right,
                    };

                    line.rect.x += logical_inline as u32;
                    if is_end {
                        line.rect.y += (*current_y - text.total_height as i32 - margin_bottom)
                            as u32
                            - system_line_height;
                    } else {
                        line.rect.y += (*current_y + margin_top) as u32;
                    }

                    if matches!(self.window.justify_content, JustifyContent::Center) {
                        if line.rect.y >= system_line_height {
                            line.rect.y -= system_line_height;
                        } else {
                            line.rect.y = 0;
                            system_line_height = 0;
                        }
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

    pub(super) fn process_background(&mut self) -> Result<(), Error> {
        let window = &self.window;
        let background_color = match &window.background_color {
            None => return Ok(()),
            Some(color) => color,
        };

        self.context.draw_background_color(*background_color)
    }
}
