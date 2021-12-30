use super::font::WHITESPACE_EM;
use super::style::KernSetting;
use ab_glyph::{point as ab_point, Font, FontArc, Glyph, PxScaleFont, Rect, ScaleFont};
use conv::ValueInto;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;
use imageproc::pixelops::weighted_sum;
use std::f32;
use std::i32;

#[derive(Clone)]
pub struct FontSetting {
    pub letter_spacing: i32,
    pub size: f32,
    pub kern_setting: KernSetting,
}

impl Default for FontSetting {
    fn default() -> FontSetting {
        FontSetting {
            size: 16.,
            letter_spacing: 0,
            kern_setting: KernSetting::Normal,
        }
    }
}

pub fn calculate_text_width(
    cur_char: char,
    next_char: Option<char>,
    font: &PxScaleFont<&FontArc>,
    rect: &Rect,
    setting: &FontSetting,
) -> i32 {
    let glyph_id = font.glyph_id(cur_char);
    if cur_char.is_whitespace() {
        return (setting.size * WHITESPACE_EM) as i32 + setting.letter_spacing;
    }

    let width = match setting.kern_setting {
        KernSetting::Normal => font.h_advance(glyph_id) as i32,
        KernSetting::Optical => rect.width() as i32,
        KernSetting::Metrics => match next_char {
            Some(next) => {
                let kern = font.kern(glyph_id, font.glyph_id(next));
                if kern == 0. {
                    font.h_advance(glyph_id) as i32
                } else {
                    (rect.width() + kern) as i32
                }
            }
            None => font.h_advance(glyph_id) as i32,
        },
    };
    width + setting.letter_spacing
}

pub fn get_glyph_rect(
    ch: char,
    font: &PxScaleFont<&FontArc>,
    setting: &FontSetting,
) -> Option<Rect> {
    let glyph_id = font.glyph_id(ch);
    let q_glyph: Glyph = glyph_id.with_scale_and_position(setting.size, ab_point(0., 0.));
    if let Some(q) = font.outline_glyph(q_glyph) {
        return Some(q.px_bounds());
    }
    None
}

/// Draws colored text on an image in place. `scale` is augmented font scaling on both the x and y axis (in pixels). Note that this function *does not* support newlines, you must do this manually
pub fn draw_text_mut<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    font: &'a FontArc,
    setting: &FontSetting,
    text: &'a str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let mut current_x = 0;
    let scaled_font = font.as_scaled(setting.size);
    let mut chars = text.char_indices().peekable();
    let whitespace = (setting.size * WHITESPACE_EM) as i32;
    while let Some((_, ch)) = chars.next() {
        if ch.is_whitespace() {
            current_x += whitespace + setting.letter_spacing;
            continue;
        }
        let glyph_id = scaled_font.glyph_id(ch);
        let q_glyph: Glyph = glyph_id.with_scale_and_position(setting.size, ab_point(0., 0.));
        if let Some(q) = scaled_font.outline_glyph(q_glyph) {
            let bb = q.px_bounds();
            q.draw(|gx, gy, gv| {
                let gx = gx as i32 + current_x + bb.min.x as i32;
                let y_bearing = (bb.min.y + scaled_font.ascent()) as i32;
                let gy = gy as i32 + y_bearing;

                let image_x = gx + x as i32;
                let image_y = gy + y as i32;

                let image_width = canvas.width() as i32;
                let image_height = canvas.height() as i32;

                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            });

            current_x += calculate_text_width(
                ch,
                chars.peek().map(|(_, ch)| *ch),
                &scaled_font,
                &bb,
                setting,
            );
        }
    }
}
