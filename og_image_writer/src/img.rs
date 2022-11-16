use crate::Error;

use super::style::BorderRadius;
use image::{
    load_from_memory_with_format, open, DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgba,
};

#[cfg(all(target_arch = "wasm32", feature = "web"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[wasm_bindgen]
pub enum ImageInputFormat {
    Png,
    Jpeg,
    // WebP,
    // Avif,
}

#[cfg(not(all(target_arch = "wasm32", feature = "web")))]
pub enum ImageInputFormat {
    Png,
    Jpeg,
    // WebP,
    // Avif,
}

impl ImageInputFormat {
    pub(super) fn as_image_format(&self) -> ImageFormat {
        match self {
            ImageInputFormat::Png => ImageFormat::Png,
            ImageInputFormat::Jpeg => ImageFormat::Jpeg,
            // ImageInputFormat::WebP => ImageFormat::WebP,
            // ImageInputFormat::Avif => ImageFormat::Avif,
        }
    }
}

pub(super) struct Size {
    pub(super) height: u32,
    pub(super) width: u32,
}

pub(super) struct ImageInfo(pub(super) ImageBuffer<Rgba<u8>, Vec<u8>>, pub(super) Size);

pub(super) fn open_and_resize(src: &str, w: u32, h: u32) -> Result<ImageInfo, Error> {
    let rgba = open(src)?.into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    Ok(ImageInfo(buffer, Size { height, width }))
}

pub(super) fn open_and_resize_with_data(
    data: &[u8],
    w: u32,
    h: u32,
    format: ImageInputFormat,
) -> Result<ImageInfo, ImageError> {
    let rgba = load_from_memory_with_format(data, format.as_image_format())?.into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    Ok(ImageInfo(buffer, Size { height, width }))
}

// See https://users.rust-lang.org/t/how-to-trim-image-to-circle-image-without-jaggy/70374
// Thanks @steffahn for sending some ideas.
pub(super) fn round(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, radius: &mut BorderRadius) {
    let (width, height) = img.dimensions();
    assert!(radius.0 + radius.1 <= width);
    assert!(radius.3 + radius.2 <= width);
    assert!(radius.0 + radius.3 <= height);
    assert!(radius.1 + radius.2 <= height);

    // top left
    border_radius(img, radius.0, |x, y| (x - 1, y - 1));
    // top right
    border_radius(img, radius.1, |x, y| (width - x, y - 1));
    // bottom right
    border_radius(img, radius.2, |x, y| (width - x, height - y));
    // bottom left
    border_radius(img, radius.3, |x, y| (x - 1, height - y));
}

fn border_radius(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    r: u32,
    coordinates: impl Fn(u32, u32) -> (u32, u32),
) {
    if r == 0 {
        return;
    }
    let r0 = r;

    // 256x antialiasing: 16x16 grid creates 256 possible shades, great for u8!
    let r = 16 * r;

    let mut x = 0;
    let mut y = r - 1;
    let mut p: i32 = 2 - r as i32;

    let mut alpha: u16 = 0;
    let mut skip_draw = true;

    let draw = |img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, alpha, x, y| {
        debug_assert!((1..=256).contains(&alpha));
        let pixel_alpha = &mut img[coordinates(r0 - x, r0 - y)].0[3];
        *pixel_alpha = ((alpha * *pixel_alpha as u16 + 128) / 256) as u8;
    };

    'l: loop {
        // Set the alpha value outside the circle to 0
        //
        // (comments for bottom_right case:)
        // remove contents below current position
        {
            let i = x / 16;
            for j in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }
        // remove contents right of current position mirrored
        {
            let j = x / 16;
            for i in y / 16 + 1..r0 {
                img[coordinates(r0 - i, r0 - j)].0[3] = 0;
            }
        }

        // draw when moving to next pixel in x-direction
        if !skip_draw {
            draw(img, alpha, x / 16 - 1, y / 16);
            draw(img, alpha, y / 16, x / 16 - 1);
            alpha = 0;
        }

        // `p` is calculated on the circle line, and alpha is calculated depends on `p`.
        for _ in 0..16 {
            skip_draw = false;

            if x >= y {
                break 'l;
            }

            alpha += y as u16 % 16 + 1;

            // When p < 0, only proceed x.
            // In the other, x and y are proceeded.
            // In this case, `p` is on the circle line.
            if p < 0 {
                // Equation for next `p`.
                x += 1;
                p += (2 * x + 2) as i32;
            } else {
                // draw when moving to next pixel in y-direction
                if y % 16 == 0 {
                    draw(img, alpha, x / 16, y / 16);
                    draw(img, alpha, y / 16, x / 16);
                    skip_draw = true;
                    alpha = (x + 1) as u16 % 16 * 16;
                }

                // Equation for next `p`.
                // x and y are proceeded at once.
                x += 1;
                p -= (2 * (y - x) + 2) as i32;
                y -= 1;
            }
        }
    }

    // one corner pixel left
    if x / 16 == y / 16 {
        // column under current position possibly not yet accounted
        if x == y {
            alpha += y as u16 % 16 + 1;
        }
        let s = y as u16 % 16 + 1;
        let alpha = 2 * alpha - s * s;
        draw(img, alpha, x / 16, y / 16);
    }

    // remove remaining square of content in the corner
    let range = y / 16 + 1..r0;
    for i in range.clone() {
        for j in range.clone() {
            img[coordinates(r0 - i, r0 - j)].0[3] = 0;
        }
    }
}
