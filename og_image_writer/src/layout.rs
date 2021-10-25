mod container;
mod img;
mod text;
mod textarea;

pub(super) use textarea::SplitText;
pub use textarea::TextArea;

use super::element::Element;
use super::style::{AlignItems, FlexDirection, JustifyContent, Margin, TextAlign};
use super::writer::OGImageWriter;
use super::Error;

impl<'a> OGImageWriter<'a> {
    pub(super) fn process(&mut self) {
        if !matches!(self.window.justify_content, JustifyContent::End) {
            self.tree.0.reverse();
        }

        let mut current_y = self.calculate_logical_block() as i32;
        let mut current_x = self.calculate_logical_inline() as i32;

        let mut tree = OGImageWriter::create_tree();
        while let Some(mut elm) = self.tree.0.pop() {
            if elm.is_absolute() {
                self.process_absolute(&mut elm);
            } else {
                match self.window.flex_direction {
                    FlexDirection::Column => self.process_column_flexbox(&mut elm, &mut current_y),
                    FlexDirection::Row => self.process_row_flexbox(&mut elm, &mut current_x),
                }
            }

            tree.0.push(elm);
        }
        self.tree.0.append(&mut tree.0);
    }

    fn calculate_logical_block(&self) -> u32 {
        let total_height = if self.content.height > self.window.height {
            self.window.height
        } else {
            self.content.height
        };

        let center_height = total_height / 2;
        let rest_height = self.window.height / 2 - center_height;

        let window_height = self.window.height;

        match &self.window.justify_content {
            JustifyContent::Start => 0,
            JustifyContent::Center => rest_height,
            JustifyContent::End => window_height,
        }
    }

    fn calculate_logical_inline(&self) -> u32 {
        let total_width = if self.content.width > self.window.width {
            self.window.width
        } else {
            self.content.width
        };

        let center_width = total_width / 2;
        let rest_width = self.window.width / 2 - center_width;

        let window_width = self.window.width;

        match &self.window.justify_content {
            JustifyContent::Start => 0,
            JustifyContent::Center => rest_width,
            JustifyContent::End => window_width,
        }
    }

    fn process_column_flexbox(&mut self, elm: &mut Element, current_y: &mut i32) {
        let window_width = self.window.width as i32;
        let is_end = matches!(self.window.justify_content, JustifyContent::End);
        match elm {
            Element::Img(Some(img)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) = img.style.margin;

                let logical_inline = match &self.window.align_items {
                    AlignItems::Start => margin_left,
                    AlignItems::Center => {
                        window_width / 2 - img.rect.width as i32 / 2 + margin_left - margin_right
                    }
                    AlignItems::End => window_width - img.rect.width as i32 - margin_right,
                };

                img.rect.x = logical_inline as u32;

                if is_end {
                    img.rect.y += (*current_y - img.rect.height as i32 - margin_bottom) as u32;
                    *current_y -= img.rect.height as i32 + margin_top + margin_bottom;
                } else {
                    img.rect.y += (*current_y + margin_top) as u32;
                    *current_y += img.rect.height as i32 + margin_top + margin_bottom;
                }
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) =
                    text.style.margin;

                let line_metrics = &text.metrics;

                // Because imageproc draw text that include line_height.
                let mut system_line_height = line_metrics.max_line_height as u32 / 2;

                for line in &mut text.lines {
                    let logical_inline = match &self.window.align_items {
                        AlignItems::Start => margin_left,
                        AlignItems::Center => {
                            window_width / 2 - line_metrics.max_line_width as i32 / 2 + margin_left
                                - margin_right
                        }
                        AlignItems::End => window_width - line_metrics.max_line_width as i32 - margin_right,
                    };

                    let content_box_inline = match text.style.text_align {
                        TextAlign::Start => 0,
                        TextAlign::Center => {
                            line_metrics.max_line_width as i32 / 2 - line.rect.width as i32 / 2
                        }
                        TextAlign::End => line_metrics.max_line_width as i32 - line.rect.width as i32,
                    } + logical_inline;

                    line.rect.x += content_box_inline as u32;
                    if is_end {
                        line.rect.y += (*current_y - line_metrics.total_height as i32 - margin_bottom)
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
                    *current_y -= line_metrics.total_height as i32 + margin_top + margin_bottom;
                } else {
                    *current_y += line_metrics.total_height as i32 + margin_top + margin_bottom;
                }
            }
            _ => {}
        }
    }

    fn process_row_flexbox(&mut self, elm: &mut Element, current_x: &mut i32) {
        let window_height = self.window.height as i32;
        let is_end = matches!(self.window.justify_content, JustifyContent::End);
        match elm {
            Element::Img(Some(img)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) = img.style.margin;

                let logical_block = match &self.window.align_items {
                    AlignItems::Start => margin_top,
                    AlignItems::Center => {
                        window_height / 2 - img.rect.height as i32 / 2 + margin_top - margin_bottom
                    }
                    AlignItems::End => window_height - img.rect.height as i32 - margin_bottom,
                };

                img.rect.y = logical_block as u32;

                if is_end {
                    img.rect.x += (*current_x - img.rect.width as i32 - margin_right) as u32;
                    *current_x -= img.rect.width as i32 + margin_left + margin_right;
                } else {
                    img.rect.x += (*current_x + margin_left) as u32;
                    *current_x += img.rect.width as i32 + margin_left + margin_right;
                }
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) =
                    text.style.margin;

                let line_metrics = &text.metrics;

                // Because imageproc draw text that include line_height.
                let mut system_line_height = line_metrics.max_line_height as u32 / 2;

                for line in &mut text.lines {
                    let logical_block = match &self.window.align_items {
                        AlignItems::Start => margin_top,
                        AlignItems::Center => {
                            window_height / 2 - line_metrics.total_height as i32 / 2 + margin_top
                                - margin_bottom
                        }
                        AlignItems::End => {
                            self.window.height as i32
                                - line_metrics.total_height as i32
                                - system_line_height as i32
                                - margin_bottom
                        }
                    };

                    line.rect.y += logical_block as u32;

                    line.rect.x += match text.style.text_align {
                        TextAlign::Start => 0,
                        TextAlign::Center => {
                            line_metrics.max_line_width as i32 / 2 - line.rect.width as i32 / 2
                        }
                        TextAlign::End => line_metrics.max_line_width as i32 - line.rect.width as i32,
                    } as u32;

                    if is_end {
                        line.rect.x +=
                            (*current_x - line_metrics.max_line_width as i32 - margin_right) as u32;
                    } else {
                        line.rect.x += (*current_x + margin_left) as u32;
                    }

                    if matches!(self.window.align_items, AlignItems::Center) {
                        if line.rect.y >= system_line_height {
                            line.rect.y -= system_line_height;
                        } else {
                            line.rect.y = 0;
                            system_line_height = 0;
                        }
                    }
                }

                if is_end {
                    *current_x -= line_metrics.max_line_width as i32 + margin_left + margin_right;
                } else {
                    *current_x += line_metrics.max_line_width as i32 + margin_left + margin_right;
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
                        self.window.width as i32 - img.rect.width as i32 - right - margin_right
                    }
                    (None, None) => margin_left,
                } as u32;
                img.rect.y += match (img.style.top, img.style.bottom) {
                    (Some(top), _) => top + margin_top,
                    (None, Some(bottom)) => {
                        self.window.height as i32 - img.rect.height as i32 - bottom - margin_bottom
                    }
                    (None, None) => margin_top,
                } as u32;
            }
            Element::Text(Some(text)) => {
                let Margin(margin_top, margin_right, margin_bottom, margin_left) =
                    text.style.margin;

                let line_metrics = &text.metrics;

                for line in &mut text.lines {
                    line.rect.x += match (text.style.left, text.style.right) {
                        (Some(left), _) => left + margin_left,
                        (None, Some(right)) => {
                            self.window.width as i32
                                - line_metrics.max_line_width as i32
                                - right
                                - margin_right
                        }
                        (None, None) => margin_left,
                    } as u32;
                    line.rect.y += match (text.style.top, text.style.bottom) {
                        (Some(top), _) => top + margin_top,
                        (None, Some(bottom)) => {
                            self.window.height as i32
                                - line_metrics.total_height as i32
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
