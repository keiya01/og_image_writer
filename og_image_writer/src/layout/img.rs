use image::ImageError;

use crate::element::{Element, Img, Rect};
use crate::img::{open_and_resize, open_and_resize_with_data, round, ImageInfo, ImageInputFormat};
use crate::style::{FlexDirection, Margin, Style};
use crate::writer::OGImageWriter;
use crate::Error;
use std::str;

impl OGImageWriter {
    pub(super) fn process_img(&mut self, img: Element, width: u32, height: u32) {
        let Margin(margin_top, margin_right, margin_bottom, margin_left) = img.margin();

        if !img.is_absolute() {
            match self.window.flex_direction {
                FlexDirection::Column => {
                    self.content.height += (height as i32 + margin_top + margin_bottom) as u32;
                }
                FlexDirection::Row => {
                    self.content.width += (width as i32 + margin_left + margin_right) as u32;
                }
            }
        }

        self.tree.0.push(img);
    }

    pub(crate) fn process_img_with_src(
        &mut self,
        src: &str,
        width: u32,
        height: u32,
        style: Style,
    ) -> Result<(), Error> {
        let ImageInfo(mut buf, size) = open_and_resize(src, width, height)?;

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone());

        let img = Element::Img(Some(Img::new(
            buf,
            Rect::new(0, 0, size.width, size.height),
            style,
        )));

        self.process_img(img, size.width, size.height);

        Ok(())
    }

    pub(crate) fn process_img_with_data(
        &mut self,
        data: &[u8],
        width: u32,
        height: u32,
        format: ImageInputFormat,
        style: Style,
    ) -> Result<(), ImageError> {
        let ImageInfo(mut buf, size) = open_and_resize_with_data(data, width, height, format)?;

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone());

        let img = Element::Img(Some(Img::new(
            buf,
            Rect::new(0, 0, size.width, size.height),
            style,
        )));

        self.process_img(img, size.width, size.height);

        Ok(())
    }
}
