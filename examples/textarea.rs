use og_image_writer::{style, writer::OGImageWriter, TextArea};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })?;

    let mut textarea = TextArea::new();
    textarea.push_text("This is ");
    textarea.push(
        "Open Graphic Image Writer",
        style::Style {
            color: style::Rgba([255, 0, 255, 255]),
            font_size: 100.,
            ..style::Style::default()
        },
        None,
    )?;
    textarea.push_text(" for ");
    textarea.push(
        "Web Developer!!!!!",
        style::Style {
            color: style::Rgba([255, 0, 0, 255]),
            font_size: 100.,
            ..style::Style::default()
        },
        Some(include_bytes!("../fonts/Roboto-Light.ttf").to_vec()),
    )?;

    writer.set_textarea(
        textarea,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            // word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            max_height: Some(150),
            text_overflow: style::TextOverflow::Ellipsis,
            word_break: style::WordBreak::BreakAll,
            ..style::Style::default()
        },
        Some(include_bytes!("../fonts/Mplus1-Black.ttf").to_vec()),
    )?;

    let out_dir = "./examples/assets";
    let out_filename = "output_textarea.png";

    writer.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
