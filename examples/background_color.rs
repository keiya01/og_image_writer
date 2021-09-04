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

    writer.set_text(
        text,
        style::Style {
            margin: style::Margin(0., 20., 100., 20.),
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

    writer.set_img(
        "./examples/assets/figure_money_satsutaba_binta.png",
        100,
        100,
        style::Style {
            text_align: style::TextAlign::End,
            ..style::Style::default()
        }
    );

    let out_dir = "./examples/assets";
    let out_filename = "output.png";

    writer.generate(&format!("{}/{}", out_dir, out_filename))?;

    Ok(())
}
