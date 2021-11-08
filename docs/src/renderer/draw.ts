import {
  BorderRadius,
  Margin,
  OGImageWriter,
  Rgba,
  Style,
  TextArea,
  WindowStyle,
} from "og_image_writer";
import type {
  Element,
  Container,
  TextAreaObj,
  ImgObj,
  Writer,
  TextAreaItem,
} from "./types";
import type { StyleObj } from "./style";
import { getDefaultStyleObj, getDefaultWindowStyleObj } from "./style";

export const createWriter = (): Writer => {
  return {
    data: [],
    style: getDefaultWindowStyleObj(),
  };
};

export const createContainer = (): Container => {
  return {
    type: "container",
    writer: createWriter(),
    style: getDefaultStyleObj(),
  };
};

export const createTextArea = (): TextAreaObj => ({
  type: "textarea",
  style: getDefaultStyleObj(),
  font: new Uint8Array(),
  area: [],
});

export const createTextAreaItem = (): TextAreaItem => ({
  text: "",
});

export const createImg = (): ImgObj => ({
  type: "img",
  data: new Uint8Array(),
  width: 0,
  height: 0,
  style: getDefaultStyleObj(),
});

export const setElement = (writer: Writer, elm: Element) => {
  writer.data.push(elm);
};

const optionalAssign = <
  T extends { [key: string]: any },
  K extends keyof T,
  V extends T[K]
>(
  target: T,
  key: K,
  value: V | undefined
) => {
  if (!value) {
    return;
  }
  target[key] = value;
};

const getStyle = (obj?: StyleObj) => {
  if (!obj) {
    return;
  }

  const style = Style.new();

  optionalAssign(
    style,
    "border_radius",
    BorderRadius.new(
      obj.borderRadius.topLeft,
      obj.borderRadius.topRight,
      obj.borderRadius.bottomLeft,
      obj.borderRadius.bottomRight
    )
  );
  optionalAssign(style, "bottom", obj.bottom);
  optionalAssign(
    style,
    "color",
    Rgba.new(obj.color.r, obj.color.g, obj.color.b, obj.color.a)
  );
  optionalAssign(style, "font_size", obj.fontSize);
  optionalAssign(style, "left", obj.left);
  optionalAssign(style, "line_height", obj.lineHeight);
  optionalAssign(
    style,
    "margin",
    Margin.new(
      obj.margin.top,
      obj.margin.right,
      obj.margin.bottom,
      obj.margin.left
    )
  );
  optionalAssign(style, "max_height", obj.maxHeight);
  optionalAssign(style, "max_width", obj.maxWidth);
  optionalAssign(style, "position", obj.position);
  optionalAssign(style, "right", obj.right);
  optionalAssign(style, "text_align", obj.textAlign);
  optionalAssign(style, "text_overflow", obj.textOverflow);
  optionalAssign(style, "top", obj.top);
  optionalAssign(style, "word_break", obj.wordBreak);

  return style;
};

export const drawImg = (writer: Writer) => {
  const windowStyle = WindowStyle.new();
  optionalAssign(windowStyle, "align_items", writer.style.alignItems);
  optionalAssign(
    windowStyle,
    "background_color",
    Rgba.new(
      writer.style.backgroundColor.r,
      writer.style.backgroundColor.g,
      writer.style.backgroundColor.b,
      writer.style.backgroundColor.a
    )
  );
  optionalAssign(windowStyle, "height", writer.style.height);
  optionalAssign(windowStyle, "flex_direction", writer.style.flexDirection);
  optionalAssign(windowStyle, "justify_content", writer.style.justifyContent);
  optionalAssign(windowStyle, "width", writer.style.width);

  const imgWriter = OGImageWriter.new(windowStyle);

  for (const elm of writer.data) {
    switch (elm.type) {
      case "textarea": {
        if (!elm.font.byteLength) {
          break;
        }

        const textarea = TextArea.new();
        elm.area.forEach((data) => {
          const style = getStyle(data.style);
          textarea.push(data.text, style, data.font);
        });
        const style = getStyle(elm.style);
        imgWriter.set_textarea(textarea, style, elm.font);
        break;
      }
      case "img": {
        if (!elm.data.byteLength) {
          break;
        }

        const style = getStyle(elm.style);
        imgWriter.set_img_with_data(elm.data, elm.width, elm.height, style);
        break;
      }
      case "container": {
        const container = drawImg(elm.writer);
        const style = getStyle(elm.style);
        imgWriter.set_container(container, style);
        break;
      }
    }
  }

  imgWriter.paint();

  console.log(imgWriter);

  return imgWriter;
};
