use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("null element kind")]
    NullElement,
    #[error("image error: {0}")]
    ImageError(#[from] ImageError),
    #[error("failed to parse invalid font bytes")]
    InvalidFontBytes,
    #[error("Container image could not found")]
    NotFoundContainerImage,
    #[error("Could not found text within range")]
    OutOfRangeText,
    #[error("Could not found specified font family")]
    NotFoundSpecifiedFontFamily,
}
