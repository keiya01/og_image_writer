//! This crate provide generating Open Graphic Image using a CSS-like API.
//!
//! And You can generate Open Graphic Image dynamically.
//!
//! - API looks like CSS.
//! - You can generate image by using template image.
//!
//! The following example generate Open Graphic Image from template PNG image.
//!
//! ```rust
//! use og_image_writer::{writer::OGImageWriter, style};
//! use std::io;
//!
//! fn main() -> io::Result<()> {
//!     let text = "This is Open Graphic Image Writer for Web Developer.";
//!
//!     let mut writer = OGImageWriter::new();
//!
//!     // Set window style.
//!     // Window acts like CSS `flexbox`. And it is specified column direction.
//!     writer.set_window_style(style::WindowStyle {
//!         width: 1024,
//!         height: 512,
//!         background_image: Some("./examples/assets/og_template.png"),
//!         align_items: style::AlignItems::Center,
//!         justify_content: style::JustifyContent::Center,
//!         ..style::WindowStyle::default()
//!     });
//!
//!     // Set text.
//!     // This text will be written into the generated Open Graphic Image.
//!     writer.set_text(text);
//!
//!     // Set text style.
//!     // Text element acts like CSS `inline-block`.
//!     writer.set_text_style(style::Style {
//!         margin: style::Margin(0., 20., 0., 20.),
//!         line_height: 1.8,
//!         font_family: "YuGothic",
//!         font_size: 50.,
//!         font_style: style::FontStyle::Normal,
//!         font_weight: style::FontWeight::Bold,
//!         word_break: style::WordBreak::Normal,
//!         color: style::RGB(1., 1., 1.),
//!         // `text_align` adjust text element position in `inline-block`
//!         text_align: style::TextAlign::Start,
//!     });
//!
//!     let out_dir = "./examples/assets";
//!     let out_filename = "output.png";
//!
//!     // Generate Open Graphic Image!
//!     writer.generate(&format!("{}/{}", out_dir, out_filename))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! For more examples, see [keiya01/og_image_writer/examples](https://github.com/keiya01/og_image_writer/tree/main/examples).
//!

pub mod style;
pub mod writer;

mod line_breaker;
