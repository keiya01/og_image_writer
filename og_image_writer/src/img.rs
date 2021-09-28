use crate::Error;

use super::style::BorderRadius;
use image::{load_from_memory, open, DynamicImage, ImageBuffer, ImageError, Rgba};
use imageproc::drawing::draw_line_segment_mut;

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
) -> Result<ImageInfo, ImageError> {
    let rgba = load_from_memory(data)?.into_rgba8();
    let buffer = DynamicImage::ImageRgba8(rgba).thumbnail(w, h).into_rgba8();
    let height = buffer.height();
    let width = buffer.width();
    Ok(ImageInfo(buffer, Size { height, width }))
}

// see: https://stackoverflow.com/questions/48478497/javascript-gecko-border-radius-adaptation-on-html-canvas-css-border-radius
// fn calculate_border_radius(r: &mut BorderRadius, w:f32, h: f32) {
//     let BorderRadius(tl, tr, bl, br) = r;
//     let max_radius_width = cmp::max(*tl + *tr, *bl + *br) as f32;
//     let max_radius_height = cmp::max(*tl + *bl, *tr + *br) as f32;
//     let width_ratio = w / max_radius_width;
//     let height_ratio = h / max_radius_height;
//     let scale_ratio = f32::min(f32::min(width_ratio, height_ratio), 1.);

//     *tl = (*tl as f32 * scale_ratio) as u32;
//     *tr = (*tr as f32 * scale_ratio) as u32;
//     *bl = (*tr as f32 * scale_ratio) as u32;
//     *br = (*tr as f32 * scale_ratio) as u32;
// }

fn while_radius<F>(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32, size: (i32, i32), draw: F)
where
    F: Fn(&mut ImageBuffer<Rgba<u8>, Vec<u8>>, (i32, i32), (i32, i32), Rgba<u8>),
{
    if r <= 0 {
        return;
    }

    let r = (r as f32 / 1.25) as i32;
    let mut x = 0i32;
    let mut y = r;

    let color = Rgba([0, 0, 0, 0]);
    let mut p = 1 - r;

    let (x0, y0) = size;

    while x <= y {
        draw(img, (x0, y0), (x, y), color);

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

fn border_top_left_radius(buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
    while_radius(buf, r - 1, (0, 0), |img, (x0, y0), (x, y), color| {
        draw_line_segment_mut(
            img,
            ((x0 + x) as f32, (y0) as f32),
            ((x0) as f32, (y0 + y) as f32),
            color,
        );

        draw_line_segment_mut(
            img,
            ((x0) as f32, (y0 + x) as f32),
            ((x0 + y) as f32, (y0) as f32),
            color,
        );
    });
}

fn border_top_right_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
    let width = img.width() as i32;

    while_radius(
        img,
        r - 1,
        (width - 1, 0),
        |img, (x0, y0), (x, y), color| {
            draw_line_segment_mut(
                img,
                ((x0 - x) as f32, (y0) as f32),
                ((x0) as f32, (y0 + y) as f32),
                color,
            );

            draw_line_segment_mut(
                img,
                ((x0) as f32, (y0 + x) as f32),
                ((x0 - y) as f32, (y0) as f32),
                color,
            );
        },
    );
}

fn border_bottom_left_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
    let height = img.height() as i32;

    while_radius(
        img,
        r - 1,
        (0, height - 1),
        |img, (x0, y0), (x, y), color| {
            draw_line_segment_mut(
                img,
                ((x0 + x) as f32, (y0) as f32),
                ((x0) as f32, (y0 - y) as f32),
                color,
            );

            draw_line_segment_mut(
                img,
                ((x0) as f32, (y0 - x) as f32),
                ((x0 + y) as f32, (y0) as f32),
                color,
            );
        },
    );
}

fn border_bottom_right_radius(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, r: i32) {
    let width = img.width() as i32;
    let height = img.height() as i32;

    while_radius(
        img,
        r - 1,
        (width - 1, height - 1),
        |img, (x0, y0), (x, y), color| {
            draw_line_segment_mut(
                img,
                ((x0 - x) as f32, (y0) as f32),
                ((x0) as f32, (y0 - y) as f32),
                color,
            );

            draw_line_segment_mut(
                img,
                ((x0) as f32, (y0 - x) as f32),
                ((x0 - y) as f32, (y0) as f32),
                color,
            );
        },
    );
}

// TODO: Support border
pub(super) fn round(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    radius: &mut BorderRadius,
    _border_width: f32,
) {
    border_top_left_radius(img, radius.0 as i32);
    border_top_right_radius(img, radius.1 as i32);
    border_bottom_right_radius(img, radius.2 as i32);
    border_bottom_left_radius(img, radius.3 as i32);
}
