use og_image_writer::{writer::OGImageWriter, style};
use std::io;

fn main() -> io::Result<()> {
    let text = "これは OGP のタイトルです。ブログのタイトルが入る予定です。Rust 言語でブラウザ作ってみた。";
    let mut writer = OGImageWriter::new(text, style::Style {
        padding_inline: 20.,
        line_height: 1.8,
        font_family: "YuGothic",
        font_size: 50.,
        font_style: style::FontStyle::Normal,
        font_weight: style::FontWeight::Bold,
        word_break: style::WordBreak::BreakAll,
        color: style::RGB(0.4, 0.5, 0.6),
    });
    writer.generate("./examples/ogp_template.png", "./examples/output.png")?;
    Ok(())
}
