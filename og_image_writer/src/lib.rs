//! This crate provide generating Open Graphic Image using a CSS-like API.
//!
//! And You can generate Open Graphic Image dynamically.
//!
//! - API looks like CSS.
//! - You can generate image by using template image.
//!
//! The following example generate Open Graphic Image from template PNG image.
//!
//!  ```rust
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
//!         font,
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
//! For more examples, see [keiya01/og_image_writer/examples](https://github.com/keiya01/og_image_writer/tree/main/examples).
//!

pub mod element;
pub mod img;
pub mod style;
pub mod writer;
pub use error::Error;

mod context;
mod error;
mod layout;
mod line_breaker;
