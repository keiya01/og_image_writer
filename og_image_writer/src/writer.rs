use super::element::{Element, Img, Text};
use super::style::{Style, WindowStyle};
use cairo::{Context, Format, ImageSurface};
use std::{fs::File, io, str};

#[derive(Default)]
pub(super) struct Content {
    pub(super) height: u32,
    pub(super) width: u32,
}

/// This struct write text to PNG.
/// You can set text or img with `set_*` method.
/// And you can set style with `set_*_style` method.
pub struct OGImageWriter<'a> {
    pub(super) context: Context,
    surface: ImageSurface,
    pub(super) tree: Vec<Element<'a>>,
    pub(super) window: WindowStyle<'a>,
    pub(super) content: Content,
}

impl<'a> OGImageWriter<'a> {
    /// Set window style. Window act like CSS `flexbox`.
    pub fn new(window: WindowStyle<'a>) -> Self {
        let surface = Self::create_surface(&window).expect("Could not create surface");
        let context = Context::new(&surface).expect("Could not initialize Context");

        let this = OGImageWriter {
            context,
            surface,
            tree: OGImageWriter::create_tree(),
            window,
            content: Content::default(),
        };

        this.process_background();

        this
    }

    pub(super) fn create_tree() -> Vec<Element<'a>> {
        Vec::with_capacity(2)
    }

    fn create_surface(window: &WindowStyle<'a>) -> io::Result<ImageSurface> {
        match window.background_image {
            Some(src) => create_surface_from_src(src),
            None => Ok(
                ImageSurface::create(Format::ARgb32, window.width, window.height)
                    .expect("Could not create surface"),
            ),
        }
    }

    /// Set text you want to write to image.
    /// And set the text element style. Text element act like CSS `inline-block`.
    pub fn set_text(&mut self, text: &'a str, style: Style<'a>) {
        self.process_text(text, style);
    }

    /// Set image you want to write to image. And set the image element style.
    pub fn set_img(&mut self, src: &'a str, width: u32, height: u32, style: Style<'a>) {
        self.process_img(src, width, height, style)
            .expect("Could not process img");
    }

    /// Generate your image.
    pub fn generate(&mut self, dest: &str) -> io::Result<()> {
        self.process();

        while let Some(elm) = self.tree.pop() {
            match elm {
                Element::Img(mut img) => self.paint_img(img.take().unwrap()),
                Element::Text(mut text) => self.paint_text(text.take().unwrap()),
            }
        }

        let mut file = File::create(dest).expect("Couldn’t create file");
        self.surface
            .write_to_png(&mut file)
            .expect("Couldn’t write to png");

        Ok(())
    }

    fn paint_img(&mut self, img: Img) {
        let stride = (img.width * 4) as i32;
        let surface = ImageSurface::create_for_data(
            img.data,
            Format::ARgb32,
            img.width as i32,
            img.height as i32,
            stride,
        )
        .expect("Could not create surface");
        self.context
            .set_source_surface(&surface, img.rect.x, img.rect.y)
            .unwrap_or_else(|_| panic!("Could not set specified image"));
        self.context.paint().expect("Could not paint image.");
    }

    fn paint_text(&self, text_elm: Text<'a>) {
        for line in &text_elm.lines {
            set_font(&self.context, &text_elm.style);
            self.context.move_to(line.rect.x, line.rect.y);
            self.context
                .show_text(&text_elm.text[line.range.clone()])
                .unwrap();
        }
    }
}

fn create_surface_from_src(src: &str) -> io::Result<ImageSurface> {
    let mut file = File::open(src)?;
    Ok(ImageSurface::create_from_png(&mut file)
        .expect("Could not create data from specified png file"))
}

pub(super) fn set_font(context: &Context, style: &Style) {
    context.select_font_face(style.font_family, style.font_style, style.font_weight);
    context.set_font_size(style.font_size);
    context.set_source_rgb(style.color.0, style.color.1, style.color.2);
}
