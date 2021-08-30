use og_image_writer::{writer::OGImageWriter, style};
use std::io;

fn main() -> io::Result<()> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new();

    writer.set_window_style(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::RGB(0.7, 0.4, 0.9)),
        ..style::WindowStyle::default()
    });

    writer.set_text(text);
    writer.set_text_style(style::Style {
        margin_inline: 20.,
        line_height: 1.8,
        font_family: "YuGothic",
        font_size: 50.,
        font_style: style::FontStyle::Normal,
        font_weight: style::FontWeight::Bold,
        word_break: style::WordBreak::Normal,
        color: style::RGB(1., 1., 1.),
    });

    writer.generate("./examples/dist/output.png")?;

    Ok(())
}
