use super::Error;

pub use rusttype::Font;

pub fn create_font<'a>(data: Vec<u8>) -> Result<Font<'a>, Error> {
    match Font::try_from_vec(data) {
        Some(font) => Ok(font),
        None => Err(Error::InvalidFontBytes),
    }
}
