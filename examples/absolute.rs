use og_image_writer::{style, writer::OGImageWriter};
use std::io;

fn main() -> io::Result<()> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::RGB(0.7, 0.4, 0.9)),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    });

    writer.set_img(
        "./examples/assets/food_sakana_hone.png",
        100,
        100,
        style::Style {
            margin: style::Margin(0., 20., 0., 20.),
            position: style::Position::Absolute,
            text_align: style::TextAlign::End,
            top: Some(20.),
            left: Some(0.),
            ..style::Style::default()
        },
    );

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0., 20., 0., 20.),
            line_height: 1.8,
            font_family: "YuGothic",
            font_size: 50.,
            font_weight: style::FontWeight::Bold,
            color: style::RGB(1., 1., 1.),
            text_align: style::TextAlign::End,
            max_height: Some(150.),
            text_overflow: style::TextOverflow::Ellipsis,
            position: style::Position::Absolute,
            bottom: Some(20.),
            right: Some(0.),
            ..style::Style::default()
        },
    );

    let out_dir = "./examples/assets";
    let out_filename = "output_absolute.png";

    writer.generate(&format!("{}/{}", out_dir, out_filename))?;

    Ok(())
}
