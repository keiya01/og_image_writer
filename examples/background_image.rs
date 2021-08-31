use og_image_writer::{writer::OGImageWriter, style};
use std::{io::{self, ErrorKind}, fs::create_dir};

fn main() -> io::Result<()> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new();

    writer.set_window_style(style::WindowStyle {
        width: 1024,
        height: 512,
        background_image: Some("./examples/assets/og_template.png"),
        align_items: style::AlignItems::End,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    });

    writer.set_text(text);
    writer.set_text_style(style::Style {
        margin: style::Margin(0., 20., 0., 20.),
        line_height: 1.8,
        font_family: "YuGothic",
        font_size: 50.,
        font_style: style::FontStyle::Normal,
        font_weight: style::FontWeight::Bold,
        word_break: style::WordBreak::Normal,
        color: style::RGB(1., 1., 1.),
        text_align: style::TextAlign::Center,
    });

    let out_dir = "./examples/dist";

    if let Result::Err(err) = create_dir(out_dir) {
        match err.kind() {
            ErrorKind::AlreadyExists => {},
            _ => return Err(err),
        }
    }

    writer.generate(&format!("{}/{}", out_dir, "output.png"))?;

    Ok(())
}
