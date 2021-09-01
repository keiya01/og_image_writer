use super::line_breaker::LineBreaker;
use super::style::{AlignItems, JustifyContent, Margin, Style, TextAlign, WindowStyle, WordBreak};
use cairo::{Context, Format, ImageSurface};
use std::{fs::File, io, str};

#[derive(Default)]
struct StyleSheet<'a> {
    window: WindowStyle<'a>,
    text: Style<'a>,
    img: Style<'a>,
}

/// This struct write text to PNG.
/// You can set text or img with `set_*` method.
/// And you can set style with `set_*_style` method.
#[derive(Default)]
pub struct OGImageWriter<'a> {
    /// Write this text to specified image.
    text: &'a str,
    /// TODO: Support img element
    img: Option<&'a str>,
    style: StyleSheet<'a>,
}

impl<'a> OGImageWriter<'a> {
    pub fn new() -> Self {
        OGImageWriter {
            text: "",
            img: None,
            style: StyleSheet {
                window: WindowStyle::default(),
                text: Style::default(),
                img: Style::default(),
            },
        }
    }

    /// Set text you want to write to image.
    pub fn set_text(&mut self, text: &'a str) {
        self.text = text;
    }

    /// TODO: Support img element.
    pub fn set_img(&mut self, img: &'a str) {
        self.img = Some(img);
    }

    /// Set window style. Window act like CSS `flexbox`.
    pub fn set_window_style(&mut self, style: WindowStyle<'a>) {
        self.style.window = style;
    }

    /// Set text element style. Text element act like CSS `inline-block`.
    pub fn set_text_style(&mut self, style: Style<'a>) {
        self.style.text = style;
    }

    /// TODO: Support img element.
    pub fn set_img_style(&mut self, style: Style<'a>) {
        self.style.text = style;
    }

    /// Generate your image.
    pub fn generate(&mut self, dest: &str) -> io::Result<()> {
        let surface = self.create_surface()?;
        let context = Context::new(&surface).expect("Could not initialize Context");

        let window_height = surface.height() as f64;
        let window_width = surface.width() as f64;

        self.process_background(&context);

        self.process_text(&context, window_width, window_height);

        let mut file = File::create(dest).expect("Couldn’t create file");
        surface
            .write_to_png(&mut file)
            .expect("Couldn’t write to png");

        Ok(())
    }

    fn create_surface(&self) -> io::Result<ImageSurface> {
        let window = &self.style.window;
        match window.background_image {
            Some(src) => {
                let mut file = File::open(src)?;
                Ok(ImageSurface::create_from_png(&mut file)
                    .expect("Could not create data from specified png file"))
            }
            None => Ok(
                ImageSurface::create(Format::ARgb32, window.width, window.height)
                    .expect("Could not create surface"),
            ),
        }
    }

    fn process_background(&self, context: &Context) {
        let window = &self.style.window;
        let background_color = match &window.background_color {
            None => return,
            Some(color) => color,
        };

        context.set_source_rgb(background_color.0, background_color.1, background_color.2);
        context
            .paint()
            .expect("Could not paint specified background_color");
    }

    fn process_text(&self, context: &Context, window_width: f64, window_height: f64) {
        let style = &self.style.text;

        let Margin(margin_top, margin_left, margin_bottom, margin_right) = style.margin;

        let text_area_width = window_width - margin_left - margin_right;

        // Initialize font metrics for line breaking.
        set_font(context, style);

        let mut line_breaker = LineBreaker::new(self.text);
        match style.word_break {
            WordBreak::Normal => line_breaker.break_text_with_whitespace(context, text_area_width),
            WordBreak::BreakAll => line_breaker.break_text_with_char(context, text_area_width),
        }

        let mut max_line_width = 0.;
        let mut total_height = 0.;
        for line in &line_breaker.lines {
            let extents = context.text_extents(&self.text[line.clone()]).unwrap();

            total_height += extents.height;

            max_line_width = if extents.x_advance > max_line_width {
                extents.x_advance
            } else {
                max_line_width
            };
        }

        let mut prev_extents_height = 0.;
        let lines_len = line_breaker.lines.len();
        for (i, line) in line_breaker.lines.into_iter().enumerate() {
            let is_first_line = i == 0;

            set_font(context, style);

            let text = &self.text[line.clone()];

            let extents = context.text_extents(text).unwrap();
            let text_height = extents.height;

            max_line_width = if extents.x_advance > max_line_width {
                extents.x_advance
            } else {
                max_line_width
            };

            let logical_block = match &self.style.window.justify_content {
                JustifyContent::Start => text_height + margin_top,
                JustifyContent::Center => (window_height - total_height) / 2. + margin_top,
                JustifyContent::End => window_height - total_height - margin_bottom,
            };

            let logical_inline = match &self.style.window.align_items {
                AlignItems::Start => margin_left,
                AlignItems::Center => window_width / 2. - max_line_width / 2.,
                AlignItems::End => window_width - max_line_width - margin_right,
            };

            let text_box_inline = match style.text_align {
                TextAlign::Start => 0.,
                TextAlign::Center => max_line_width / 2. - extents.x_advance / 2.,
                TextAlign::End => max_line_width - extents.x_advance,
            } + logical_inline;

            if lines_len == 1 {
                context.move_to(text_box_inline, logical_block);
                context.show_text(text).unwrap();
                break;
            }

            let line_height = text_height * style.line_height / 2.;

            let pos_y = logical_block + prev_extents_height;
            let pos_y = if !is_first_line {
                pos_y + line_height
            } else {
                pos_y
            };

            prev_extents_height += if !is_first_line {
                text_height + line_height
            } else {
                text_height
            };

            context.move_to(text_box_inline, pos_y);
            context.show_text(text).unwrap();
        }
    }
}

fn set_font(context: &Context, style: &Style) {
    context.select_font_face(style.font_family, style.font_style, style.font_weight);
    context.set_font_size(style.font_size);
    context.set_source_rgb(style.color.0, style.color.1, style.color.2);
}
