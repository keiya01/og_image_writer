use dev::components::*;
use image::RgbaImage;
use std::env::current_dir;
use std::fs::read;

// refer https://github.com/alexheretic/ab-glyph/blob/cee6e8a15b032ca90628717e52c9f987fb1875e9/dev/tests/render_reference.rs#L11-L36
#[cfg(test)]
fn assert_image(expected: &[u8], actual: RgbaImage) {
    let new_image = actual;

    let reference = image::load_from_memory_with_format(expected, image::ImageFormat::Png)
        .expect("!image::load")
        .to_rgba8();

    assert_eq!(reference.dimensions(), new_image.dimensions());

    for y in 0..reference.height() {
        for x in 0..reference.width() {
            assert_eq!(
                reference.get_pixel(x, y),
                new_image.get_pixel(x, y),
                "unexpected alpha difference at ({}, {})",
                x,
                y
            );
        }
    }
}

macro_rules! assert_component {
    ($component_name:ident) => {{
        let mut w = $component_name().unwrap();
        w.paint().unwrap();
        let rgba = w.into_rgba().unwrap();
        let path = format!(
            "{}/snapshots/output_{}.png",
            current_dir().unwrap().display(),
            stringify!($component_name)
        );
        assert_image(&read(path).unwrap(), rgba);
    }};
}

#[test]
fn compare_absolute() {
    assert_component!(absolute);
}

#[test]
fn compare_background_color() {
    assert_component!(background_color);
}

#[test]
fn compare_background_image() {
    assert_component!(background_image);
}

#[test]
fn compare_container() {
    assert_component!(container);
}

#[test]
fn compare_ellipsis() {
    assert_component!(ellipsis);
}

#[test]
fn compare_font_context() {
    assert_component!(font_context);
}

#[test]
fn compare_font_kern() {
    assert_component!(font_kern);
}
#[test]
fn compare_row_container() {
    assert_component!(row_container);
}

#[test]
fn compare_textarea() {
    assert_component!(textarea);
}

#[test]
fn compare_white_space() {
    assert_component!(white_space);
}
