use image::{open, DynamicImage, ImageBuffer, Rgba, load_from_memory};

pub(super) struct Size {
    pub(super) height: u32,
    pub(super) width: u32,
}

pub(super) fn open_and_resize(src: &str, w: u32, h: u32) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, Size) {
    let rgba = open(src)
        .expect("Could not load specified image.")
        .into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    (buffer, Size { height, width })
}

pub(super) fn open_and_resize_with_data(data: &[u8], w: u32, h: u32) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, Size) {
    let rgba = load_from_memory(data).expect("Could not load specified image").into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    (buffer, Size { height, width })
}
