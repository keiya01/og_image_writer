use og_image_writer::{img::ImageInputFormat, style, writer::OGImageWriter, Error};

pub fn absolute() -> Result<OGImageWriter, Error> {
    let text = "This is Open Graphic Image Writer for Web Developer.";

    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Center,
        justify_content: style::JustifyContent::Center,
        ..style::WindowStyle::default()
    })?;

    writer.set_img_with_data(
        include_bytes!("../../../assets/thumbnail_circle.png"),
        100,
        100,
        ImageInputFormat::Png,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            position: style::Position::Absolute,
            text_align: style::TextAlign::End,
            top: Some(20),
            left: Some(0),
            border_radius: style::BorderRadius(50, 50, 50, 50),
            ..style::Style::default()
        },
    )?;

    let font = Vec::from(include_bytes!("../../../fonts/Mplus1-Black.ttf") as &[u8]);

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0, 20, 0, 20),
            line_height: 1.8,
            font_size: 50.,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::End,
            max_height: Some(150),
            text_overflow: style::TextOverflow::Ellipsis,
            position: style::Position::Absolute,
            bottom: Some(0),
            right: Some(0),
            ..style::Style::default()
        },
        Some(font),
    )?;

    Ok(writer)
}
