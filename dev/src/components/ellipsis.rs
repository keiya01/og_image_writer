use og_image_writer::{style, writer::OGImageWriter, Error};

pub fn ellipsis() -> Result<OGImageWriter, Error> {
    let text = "This is Open Graphic Image Writer for Web Developer. This is multi line text, but this text is omitted with ellipsis.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })?;

    let font = Vec::from(include_bytes!("../../../fonts/Mplus1-Black.ttf") as &[u8]);

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 100.,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            max_height: Some(150),
            text_overflow: style::TextOverflow::Ellipsis,
            ..style::Style::default()
        },
        Some(font),
    )?;

    Ok(writer)
}
