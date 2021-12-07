import type { OGImageWriter } from "og_image_writer";
import { ImageOutputFormat, ImageOutputFormatOption } from "og_image_writer";

export const renderImg = (img: HTMLImageElement, writer: OGImageWriter) => {
  const data = writer.encode(
    ImageOutputFormat.Png,
    ImageOutputFormatOption.new()
  );
  img.src = URL.createObjectURL(new Blob([data.buffer], { type: "image/png" }));
};
