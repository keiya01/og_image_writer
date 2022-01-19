use crate::element::{Element, Img, Rect};
use crate::img::round;
use crate::style::Style;
use crate::writer::OGImageWriter;
use crate::Error;

impl OGImageWriter {
    pub(crate) fn process_container(
        &mut self,
        writer: &mut OGImageWriter,
        style: Style,
    ) -> Result<(), Error> {
        let mut image = match writer.context.image.take() {
            Some(image) => image,
            None => return Err(Error::NotFoundContainerImage),
        };

        // TODO: support border for image
        round(&mut image, &mut style.border_radius.clone());

        let img = Element::Img(Some(Img::new(
            image,
            Rect::new(0, 0, writer.window.width, writer.window.height),
            style,
        )));

        self.process_img(img, writer.window.width, writer.window.height);
        Ok(())
    }
}
