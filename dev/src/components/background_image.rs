use og_image_writer::{img::ImageInputFormat, style, writer::OGImageWriter, Error};

pub fn background_image() -> Result<OGImageWriter, Error> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::from_data(
        style::WindowStyle {
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        },
        include_bytes!("../../../assets/og_template.png"),
        ImageInputFormat::Png,
    )?;

    let font = Vec::from(include_bytes!("../../../fonts/Mplus1-Black.ttf") as &[u8]);

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            word_break: style::WordBreak::Normal,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            ..style::Style::default()
        },
        Some(font),
    )?;

    Ok(writer)
}
