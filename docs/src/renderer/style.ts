import {
  AlignItems,
  FlexDirection,
  JustifyContent,
  Position,
  TextAlign,
  WordBreak,
  WhiteSpace,
} from "og_image_writer";

export type BorderRadiusObj = {
  topLeft: number;
  topRight: number;
  bottomLeft: number;
  bottomRight: number;
};

export type MarginObj = {
  top: number;
  right: number;
  bottom: number;
  left: number;
};

export type RgbaObj = {
  r: number;
  g: number;
  b: number;
  a: number;
};

export type StyleObj = {
  borderRadius?: BorderRadiusObj;
  bottom?: number;
  color?: RgbaObj;
  fontSize?: number;
  left?: number;
  lineHeight?: number;
  margin?: MarginObj;
  maxHeight?: number;
  maxWidth?: number;
  position?: Position;
  right?: number;
  textAlign?: TextAlign;
  textOverflow?: string;
  top?: number;
  wordBreak?: WordBreak;
  whiteSpace?: WhiteSpace;
};

export const getDefaultStyleObj = (): StyleObj => ({
  borderRadius: { topLeft: 0, topRight: 0, bottomLeft: 0, bottomRight: 0 },
  color: { r: 0, g: 0, b: 0, a: 255 },
  fontSize: 30,
  lineHeight: 1.5,
  margin: { top: 0, right: 0, bottom: 0, left: 0 },
  position: Position.Static,
  textAlign: TextAlign.Start,
  textOverflow: "clip",
  wordBreak: WordBreak.Normal,
  whiteSpace: WhiteSpace.Normal,
});

export type WindowStyleObj = {
  alignItems?: AlignItems;
  backgroundColor?: RgbaObj;
  flexDirection?: FlexDirection;
  height?: number;
  justifyContent?: JustifyContent;
  width?: number;
};

export const getDefaultWindowStyleObj = (): WindowStyleObj => ({
  alignItems: AlignItems.Start,
  backgroundColor: { r: 0, g: 0, b: 0, a: 255 },
  flexDirection: FlexDirection.Column,
  height: 0,
  justifyContent: JustifyContent.Start,
  width: 0,
});
