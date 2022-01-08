use og_image_writer::{style, writer::OGImageWriter, Error};

pub fn font_kern() -> Result<OGImageWriter, Error> {
    let text = "これは、画像を動的に作るためのツールです。";

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
            word_break: style::WordBreak::BreakAll,
            color: style::Rgba([255, 255, 255, 255]),
            text_align: style::TextAlign::Start,
            letter_spacing: 10,
            kern_setting: style::KernSetting::Optical,
            ..style::Style::default()
        },
        Some(font),
    )?;

    Ok(writer)
}
