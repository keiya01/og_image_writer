use image::ImageError;

use crate::element::{Element, Img, Rect};
use crate::img::{open_and_resize, open_and_resize_with_data, round, ImageInfo};
use crate::style::Style;
use crate::writer::OGImageWriter;
use crate::Error;
use std::str;

impl<'a> OGImageWriter<'a> {
    pub(super) fn process_img(&mut self, img: Element<'a>, width: u32, height: u32) {
        if !img.is_absolute() {
            self.content.height += height;
            self.content.width = if self.content.width > width {
                self.content.width
            } else {
                width
            };
        }

        self.tree.push(img);
    }

    pub(crate) fn process_img_with_src(
        &mut self,
        src: &'a str,
        width: u32,
        height: u32,
        style: Style<'a>,
    ) -> Result<(), Error> {
        let ImageInfo(mut buf, size) = open_and_resize(src, width, height)?;

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone(), 0.);

        let img = Element::Img(Some(Img::new(
            buf,
            size.width,
            size.height,
            Rect::new(0, 0),
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
        style: Style<'a>,
    ) -> Result<(), ImageError> {
        let ImageInfo(mut buf, size) = open_and_resize_with_data(data, width, height)?;

        // TODO: support border for image
        round(&mut buf, &mut style.border_radius.clone(), 0.);

        let img = Element::Img(Some(Img::new(
            buf,
            size.width,
            size.height,
            Rect::new(0, 0),
            style,
        )));

        self.process_img(img, size.width, size.height);

        Ok(())
    }
}
