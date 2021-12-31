//! This crate provide generating Open Graphic Image using a CSS-like API.
//!
//! And You can generate Open Graphic Image dynamically.
//!
//! - API looks like CSS.
//! - You can generate image by using template image.
//!
//! The following example generate Open Graphic Image from template image.
//!
//! ```rust
//! use og_image_writer::{style, writer::OGImageWriter};
//! use std::path::Path;
//!
//! fn main() -> anyhow::Result<()> {
//!     let text = "This is Open Graphic Image Writer for Web Developer.";
//!
//!     let mut writer = OGImageWriter::from_data(
//!         style::WindowStyle {
//!             align_items: style::AlignItems::Center,
//!             justify_content: style::JustifyContent::Center,
//!             ..style::WindowStyle::default()
//!         },
//!         include_bytes!("../../examples/assets/og_template.png"),
//!     )?;
//!
//!     let font = Vec::from(include_bytes!("../../fonts/Mplus1-Black.ttf") as &[u8]);
//!
//!     writer.set_text(
//!         text,
//!         style::Style {
//!             margin: style::Margin(0, 20, 0, 20),
//!             line_height: 1.8,
//!             font_size: 100.,
//!             word_break: style::WordBreak::Normal,
//!             color: style::Rgba([255, 255, 255, 255]),
//!             text_align: style::TextAlign::Start,
//!             ..style::Style::default()
//!         },
//!         Some(font),
//!     )?;
//!
//!     let out_dir = "../examples/assets";
//!     let out_filename = "output_background_image.png";
//!
//!     writer.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! You can also create custom image like bellow.
//!
//! ```rust
//! use og_image_writer::{style, writer::OGImageWriter, TextArea, font::FontContext};
//! use std::path::Path;
//!
//! fn main() -> anyhow::Result<()> {
//!    let mut writer = OGImageWriter::new(style::WindowStyle {
//!        width: 1024,
//!        height: 512,
//!        background_color: Some(style::Rgba([70, 40, 90, 255])),
//!        align_items: style::AlignItems::Center,
//!        justify_content: style::JustifyContent::Center,
//!        ..style::WindowStyle::default()
//!    })?;
//!
//!    // Set global fallback fonts.
//!    let mut fc = FontContext::new();
//!    fc.push(Vec::from(include_bytes!("../../fonts/Mplus1-Black.ttf") as &[u8]))?;
//!
//!    // Set style for each text.
//!    let mut textarea = TextArea::new();
//!    // Japanese font will be replaced with specified fallback font.
//!    textarea.push_text("こんにちは。 ");
//!    textarea.push_text("This is ");
//!    textarea.push(
//!        "Open Graphic Image Writer",
//!        style::Style {
//!            color: style::Rgba([255, 0, 255, 255]),
//!            font_size: 100.,
//!            ..style::Style::default()
//!        },
//!        None,
//!    )?;
//!    textarea.push_text(" for ");
//!    textarea.push(
//!        // Japanese font will be replaced with specified fallback font.
//!        "Web Developer(Web開発者)",
//!        style::Style {
//!            color: style::Rgba([255, 0, 0, 255]),
//!            font_size: 100.,
//!            ..style::Style::default()
//!        },
//!        Some(Vec::from(include_bytes!("../../fonts/Roboto-Light.ttf") as &[u8])),
//!    )?;
//!    textarea.push_text("!!!");
//!
//!    writer.set_textarea(
//!        textarea,
//!        style::Style {
//!            margin: style::Margin(0, 20, 0, 20),
//!            line_height: 1.8,
//!            font_size: 100.,
//!            color: style::Rgba([255, 255, 255, 255]),
//!            text_align: style::TextAlign::Start,
//!            word_break: style::WordBreak::BreakAll,
//!            ..style::Style::default()
//!        },
//!        Some(Vec::from(include_bytes!("../../fonts/OpenSansCondensed-Light.ttf") as &[u8])),
//!    )?;
//!
//!    let out_dir = "../examples/assets";
//!    let out_filename = "font_context.png";
//!
//!    writer.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;
//!
//!    Ok(())
//! }
//! ```
//!
//! For more examples, see [keiya01/og_image_writer/examples](https://github.com/keiya01/og_image_writer/tree/main/examples).
//!

pub mod element;
pub mod font;
pub mod img;
pub mod style;
pub mod writer;
pub use context::ImageOutputFormat;
pub use error::Error;
pub use layout::TextArea;

mod context;
mod error;
mod glyph;
mod layout;
mod line_breaker;
mod renderer;
