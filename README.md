# Open Graphic Image Writer

[Documentation](https://crates.io/crates/og_image_writer)

You can generate Open Graphic Image dynamically.

- A CSS-like API.
- You can generate image by using template image.
- WASM/WASI support.

## Problem

Currently, when you want to create OGP image dynamically, you may use canvas. But to use canvas, you need to open a browser and run some script.
This is overhead, especially if you are using feature like the SSG.
Therefore this lib is targeting a high performance API for all platform by using wasm.

## Example

For more examples, see [keiya01/og_image_writer/examples](https://github.com/keiya01/og_image_writer/tree/main/examples).

```rust
use og_image_writer::{style, writer::OGImageWriter};

fn main() -> anyhow::Result<()> {
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
            ..style::Style::default()
        },
    )?;

    let out_dir = "./examples/assets";
    let out_filename = "output_background_image.png";

    writer.generate(&format!("{}/{}", out_dir, out_filename))?;

    Ok(())
}
```

When you pass the following the template image,

![example template image](https://raw.githubusercontent.com/keiya01/og_image_writer/main/examples/assets/og_template.png)

this code generate the following image.

![example output image](https://raw.githubusercontent.com/keiya01/og_image_writer/main/examples/assets/output_background_image.png)

And you can also set `background-color` instead of passing the template image.

For more examples, see [keiya01/og_image_writer/examples](https://github.com/keiya01/og_image_writer/tree/main/examples).
