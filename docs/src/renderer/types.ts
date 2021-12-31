import type { StyleObj, WindowStyleObj } from "./style";
import type { ImageInputFormat } from "og_image_writer";

export type TextAreaItem = {
  text: string;
  style?: StyleObj;
  font?: Uint8Array;
};

export type TextAreaObj = {
  type: "textarea";
  style: StyleObj;
  font?: Uint8Array;
  area: TextAreaItem[];
};

export type ImgObj = {
  type: "img";
  data: Uint8Array;
  width: number;
  height: number;
  format: ImageInputFormat;
  style: StyleObj;
};

export type Element = TextAreaObj | ImgObj | Container;

export type Container = {
  type: "container";
  writer: Writer;
  style: StyleObj;
};

export type FontContextObj = {
  context: Uint8Array[];
};

export type Writer = {
  data: Element[];
  style: WindowStyleObj;
  fontContext: FontContextObj;
};
