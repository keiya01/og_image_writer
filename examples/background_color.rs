use og_image_writer::{style, writer::OGImageWriter};
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let text =
        "This is Open Graphic Image Writer for Web Developer. Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba([70, 40, 90, 100])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    });

    let font = Vec::from(include_bytes!("../fonts/Mplus1-Black.ttf") as &[u8]);

    writer.set_img(
        "./examples/assets/food_sakana_hone.png",
        100,
        100,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            text_align: style::TextAlign::End,
            ..style::Style::default()
        },
    );

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 100]),
            text_align: style::TextAlign::Start,
            ..style::Style::default()
        },
        font,
    );

    let out_dir = "./examples/assets";
    let out_filename = "output_background_color.png";

    writer.generate(Path::new(&format!("{}/{}", out_dir, out_filename)));

    Ok(())
}
