use super::char::RenderingCharIndices;
use super::font::whitespace_width;
use super::font_trait::Font;
use super::style::KernSetting;
use ab_glyph::{point, Glyph};
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
    pub is_pre: bool,
}

impl Default for FontSetting {
    fn default() -> FontSetting {
        FontSetting {
            size: 16.,
            letter_spacing: 0,
            kern_setting: KernSetting::Normal,
            is_pre: false,
        }
    }
}

/// Draws colored text on an image in place. `scale` is augmented font scaling on both the x and y axis (in pixels). Note that this function *does not* support newlines, you must do this manually
pub(super) fn draw_text_mut<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    font: &'a dyn Font,
    setting: &FontSetting,
    text: &'a str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let mut current_x = 0;
    let mut chars = RenderingCharIndices::from_str(text);
    let whitespace = whitespace_width(setting.size) as i32;
    while let Some((flags, _, ch, _)) = chars.next() {
        let peek_char = chars.peek_char();

        if ch.is_whitespace() {
            if peek_char.is_some() {
                current_x += whitespace + setting.letter_spacing;
            }
            continue;
        }

        let glyph_id = font.glyph_id(ch);
        let q_glyph: Glyph = glyph_id.with_scale_and_position(setting.size, point(0., 0.));
        if let Some(q) = font.outline_glyph(q_glyph, setting.size) {
            let bb = q.px_bounds();
            q.draw(|gx, gy, gv| {
                let mut gx = gx as i32 + current_x;
                if let KernSetting::Normal = setting.kern_setting {
                    gx += bb.min.x as i32;
                }

                let y_bearing = (bb.min.y + font.ascent(setting.size)) as i32;
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

            current_x += font.calculate_text_width(ch, peek_char, &flags, &bb, setting);
        }
    }
}
