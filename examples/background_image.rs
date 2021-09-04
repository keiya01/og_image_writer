use og_image_writer::{style, writer::OGImageWriter};
use std::io;

fn main() -> io::Result<()> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_image: Some("./examples/assets/og_template.png"),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    });

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0., 20., 0., 20.),
            line_height: 1.8,
            font_family: "YuGothic",
            font_size: 50.,
            font_style: style::FontStyle::Normal,
            font_weight: style::FontWeight::Bold,
            word_break: style::WordBreak::Normal,
            color: style::RGB(1., 1., 1.),
            text_align: style::TextAlign::Start,
        },
    );

    let out_dir = "./examples/assets";
    let out_filename = "output.png";

    writer.generate(&format!("{}/{}", out_dir, out_filename))?;

    Ok(())
}
