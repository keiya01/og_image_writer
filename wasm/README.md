# Open Graphic Image Writer For Web Assembly

This package is the implementation of [og_image_writer](https://github.com/keiya01/og_image_writer) for wasm.
You can generate custom image by using this package.

## Example

```js
import init, {
  AlignItems,
  JustifyContent,
  OGImageWriter,
  Rgba,
  Style,
  TextArea,
  WindowStyle,
} from "og_image_writer";

const createRgba = (r, g, b, a) => {
  let color = Rgba.new(r, g, b, a);
  return color;
};

const fetchFont = (path) => {
  return fetch(path)
    .then((res) => {
      return res.arrayBuffer();
    })
    .then((data) => new Uint8Array(data));
};

const textarea = () => {
  const textarea = TextArea.new();

  const style1 = Style.new();
  style1.color = createRgba(0, 100, 50, 255);
  style1.font_size = 100;
  textarea.push("Hello", style1);

  const style2 = Style.new();
  style2.color = createRgba(20, 25, 255, 255);
  style2.font_size = 100;
  textarea.push(" World", style2);

  return textarea;
};

const generateImage = async (w, h) => {
  const windowStyle = WindowStyle.new();
  windowStyle.background_color = createRgba(255, 255, 100, 255);
  windowStyle.align_items = AlignItems.Center;
  windowStyle.justify_content = JustifyContent.Center;
  windowStyle.width = w;
  windowStyle.height = h;

  const writer = OGImageWriter.new(windowStyle);

  const textareaStyle = Style.new();
  writer.set_textarea(
    textarea(),
    textareaStyle,
    await fetchFont("/example.ttf")
  );

  writer.paint();

  return writer.into_vec();
};

const render = async (w, h) => {
  // initialize wasm module
  await init();

  const data = await generateImage(w, h);

  const canvas = document.querySelector("canvas");
  const ctx = canvas.getContext("2d");
  let imageData = new ImageData(new Uint8ClampedArray(data.buffer), w, h);

  ctx.putImageData(imageData, 0, 0);
};

render(800, 500);
```

## Usage

### OGImageWriter

```ts
export class OGImageWriter {
  static new(style: WindowStyle): OGImageWriter;

  /**
   * create instance with specified background-image.
   * `data` is specified background-image data.
   */
  static from_data(style: WindowStyle, data: Uint8Array): OGImageWriter;

  set_text(text: string, style: Style, font: Uint8Array): void;

  /**
   * TextArea can have chunked text.
   * For example you can set different color one by one like `<span>` element.
   * `style` is parent style. If text element has not style, it's style is replaced with parent style.
   * `font` is parent font. If text element has not font, it's font is replaced with parent font.
   */
  set_textarea(textarea: TextArea, style: Style, font: Uint8Array): void;

  set_img_with_data(
    data: Uint8Array,
    width: number,
    height: number,
    style: Style
  ): void;

  /**
   * Set other OGImageWriter instance to parent OGImageWriter.
   */
  set_container(writer: OGImageWriter, style: Style): void;

  /**
   * Paint OGImageWriter with current element.
   * This method must be called before you call `set_container` method.
   */
  paint(): void;

  /**
   * You can set returned value to ImageData.
   */
  into_vec(): Uint8Array;
}

/**
 * This element is used with `set_textarea` OGImageWriter method.
 */
export class TextArea {
  static new(): TextArea;

  push(text: string, style?: Style, font?: Uint8Array): void;
}
```

### Style

```ts
export class Style {
  static new(): Style;

  border_radius: BorderRadius;
  bottom?: number;
  color: Rgba;
  font_size: number;
  left?: number;
  line_height: number;
  margin: Margin;
  /**
   * For Text element
   */
  max_height?: number;
  /**
   * For Text element
   */
  max_width?: number;
  position: Position;
  right?: number;
  /**
   * For Text element
   */
  text_align: TextAlign;
  text_overflow: TextOverflow;
  top?: number;
  /**
   * For Text element
   */
  word_break: WordBreak;
}
```

### Window Style

```ts
export class WindowStyle {
  static new(): WindowStyle;

  align_items: AlignItems;
  /**
   * NOTE: This property is undefined in default.
   */
  background_color?: Rgba;
  /**
   * This controls the direction in which the children of a node are laid out.
   */
  flex_direction: FlexDirection;
  height: number;
  justify_content: JustifyContent;
  width: number;
}
```

### Style Properties

```ts
/**
 * Adjust the horizontal position.
 * default: Start
 */
export enum AlignItems {
  Start,
  Center,
  End,
}

/**
 * default: all `0`
 */
export class BorderRadius {
  static new(tl: number, tr: number, bl: number, br: number): BorderRadius;

  bottom_left: number;
  bottom_right: number;
  top_left: number;
  top_right: number;
}

/**
 * default: Column
 */
export enum FlexDirection {
  Column,
  Row,
}

/**
 * Adjust the vertical position.
 * default: Start
 */
export enum JustifyContent {
  Start,
  Center,
  End,
}

/**
 * default: all `0`
 */
export class Margin {
  static new(top: number, right: number, bottom: number, left: number): Margin;

  bottom: number;
  left: number;
  right: number;
  top: number;
}

/**
 * default: Static
 */
export enum Position {
  Static,
  Absolute,
}

/**
 * default: r=0, g=0, b=0, a=255
 */
export class Rgba {
  static new(r: number, g: number, b: number, a: number): Rgba;
}

/**
 * Adjust the text horizontal position.
 */
export enum TextAlign {
  Start,
  Center,
  End,
}

/**
 * default: "clip"
 */
type TextOverflow = "clip" | "ellipsis" | string;

/**
 * default: Normal
 */
export enum WordBreak {
  Normal,
  BreakAll,
}
```
