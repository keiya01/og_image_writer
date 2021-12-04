import type { OGImageWriter } from "og_image_writer";

export const renderImg = (
  canvas: HTMLCanvasElement,
  writer: OGImageWriter,
  w: number,
  h: number
) => {
  canvas.width = w;
  canvas.height = h;

  const ctx = canvas.getContext("2d");
  const data = writer.into_vec();
  const imageData = new ImageData(new Uint8ClampedArray(data.buffer), w, h);

  ctx.putImageData(imageData, 0, 0);
};
