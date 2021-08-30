use cairo::{ImageSurface, Context, Format};
use std::{fs::File, io, ops::Range, str};
use super::style::{WordBreak, Style, WindowStyle};

struct LineBreaker<'a> {
    title: &'a str,
    lines: Vec<Range<usize>>,
}

// TODO: support truncate text when overflow specified height.
impl<'a> LineBreaker<'a> {
    fn new(title: &'a str) -> Self {
        LineBreaker {
            title,
            lines: vec![],
        }
    }

    // TODO: support hyphenation
    fn break_text_with_whitespace(&mut self, context: &Context, width: f64) {
        let text_arr: Vec<&str> = self.title.split_whitespace().collect();

        let text_arr_len = text_arr.len();

        let whitespace_width = context.text_extents(" ").unwrap().x_advance;
        let whitespace_idx = 1;

        let mut line = 0..0;
        let mut line_width = 0.;
        for (i, text) in text_arr.into_iter().enumerate() {
            let extents = context.text_extents(text).unwrap();

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

    fn break_text_with_char(&mut self, context: &Context, width: f64) {
        let chars = self.title.char_indices();

        let mut line = 0..0;
        let mut line_width = 0.;
        for (i, ch) in chars.into_iter() {
            let extents = context.text_extents(&ch.to_string()).unwrap();

            let ch_width = extents.x_advance;

            if width <= line_width + ch_width {
                let start = line.end;
                self.lines.push(line);
                line = start..start;
                line_width = 0.;
            }

            line.end = i;
            line_width += ch_width;
        }

        self.lines.push(line);
    }
}

#[derive(Default)]
struct StyleSheet<'a> {
    window: WindowStyle<'a>,
    text: Style<'a>,
    img: Style<'a>,
}

#[derive(Default)]
pub struct OGImageWriter<'a> {
    text: &'a str,
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

    pub fn set_text(&mut self, text: &'a str) {
        self.text = text;
    }

    pub fn set_img(&mut self, img: &'a str) {
        self.img = Some(img);
    }

    pub fn set_window_style(&mut self, style: WindowStyle<'a>)  {
        self.style.window = style;
    }

    pub fn set_text_style(&mut self, style: Style<'a>)  {
        self.style.text = style;
    }

    pub fn set_img_style(&mut self, style: Style<'a>)  {
        self.style.text = style;
    }

    pub fn generate(&mut self, dest: &str) -> io::Result<()> {
        let surface = self.create_surface()?;
        let context = Context::new(&surface).expect("Could not initialize Context");

        let window_height = surface.height() as f64;
        let window_width = surface.width() as f64;

        self.process_background(&context);

        self.process_text(&context, window_width, window_height);

        let mut file = File::create(dest)
            .expect("Couldn’t create file");
        surface.write_to_png(&mut file)
            .expect("Couldn’t write to png");

        Ok(())
    }

    fn create_surface(&self) -> io::Result<ImageSurface> {
        let window = &self.style.window;
        match window.background_image {
            Some(src) => {
                let mut file = File::open(src)?;
                Ok(ImageSurface::create_from_png(&mut file).expect("Could not create data from specified png file"))
            },
            None => {
                Ok(ImageSurface::create(Format::ARgb32, window.width, window.height).expect("Could not create surface"))
            }
        }
    }

    fn process_background(&self, context: &Context) {
        let window = &self.style.window;
        let background_color = match &window.background_color {
            None => return,
            Some(color) => color,
        };

        context.set_source_rgb(background_color.0, background_color.1, background_color.2);
        context.paint().expect("Could not paint specified background_color");
    }

    fn process_text(&self, context: &Context, window_width: f64, window_height: f64) {
        let style = &self.style.text;

        let text_area_width = window_width - style.margin_inline * 2.;

        // Initialize font metrics for line breaking.
        set_font(&context, style);

        let mut line_breaker = LineBreaker::new(self.text);
        match style.word_break {
            WordBreak::Normal => line_breaker.break_text_with_whitespace(&context, text_area_width),
            WordBreak::BreakAll => line_breaker.break_text_with_char(&context, text_area_width),
        }

        let mut total_height = 0.;
        for line in &line_breaker.lines {
            let extents = context.text_extents(&self.text[line.clone()]).unwrap();
            total_height += extents.height;
        }

        let mut prev_extents_height = 0.;
        let lines_len = line_breaker.lines.len();
        for (i, line) in line_breaker.lines.into_iter().enumerate() {
            let is_first_line = i == 0;

            set_font(&context, style);

            let text = &self.text[line.clone()];

            let extents = context.text_extents(text).unwrap();
            let text_height = extents.height;

            let line_height = text_height * style.line_height / 2.;

            let pos_y = ((window_height - total_height) / 2.) - text_height / 2. + prev_extents_height;
            let pos_y = if !is_first_line {
                pos_y + line_height
            } else {
                pos_y
            };

            prev_extents_height += if !is_first_line  {
                text_height + line_height
            } else {
                text_height
            };

            if lines_len == 1 {
                context.move_to(window_width / 2. - extents.width / 2., window_height / 2. - text_height / 2.);
                context.show_text(text).unwrap();
                break;
            }

            context.move_to(style.margin_inline, pos_y);
            context.show_text(text).unwrap();
        }
    }
}
fn set_font(context: &Context, style: &Style) {
    context.select_font_face(style.font_family, style.font_style, style.font_weight);
    context.set_font_size(style.font_size);
    context.set_source_rgb(style.color.0, style.color.1, style.color.2);
}
