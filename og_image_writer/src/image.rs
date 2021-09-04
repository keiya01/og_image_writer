use image::imageops::FilterType;
use image::{open, DynamicImage};
use std::io;

pub(super) struct Size {
    pub(super) height: u32,
    pub(super) width: u32,
}

pub(super) fn open_and_resize(src: &str, w: u32, h: u32) -> io::Result<(Vec<u8>, Size)> {
    let rgba = open(src)
        .expect("Could not load specified image.")
        .into_bgra8();
    let buffer = DynamicImage::ImageBgra8(rgba)
        .resize(w, h, FilterType::Triangle)
        .into_bgra8();
    let height = buffer.height();
    let width = buffer.width();
    Ok((buffer.into_vec(), Size { height, width }))
}
