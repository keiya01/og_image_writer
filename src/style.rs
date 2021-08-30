pub use cairo::{ FontSlant as FontStyle, FontWeight };

pub enum WordBreak {
  Normal,
  BreakAll,
}

pub struct RGB(pub f64, pub f64, pub f64);

pub struct Style<'a> {
  pub padding_inline: f64,
  pub line_height: f64,
  pub font_size: f64,
  pub font_family: &'a str,
  pub font_style: FontStyle,
  pub font_weight: FontWeight,
  pub word_break: WordBreak,
  pub color: RGB,
  // TODO: support dynamic background_color
  // pub background_color: RGB,
}

impl<'a> Default for Style<'a> {
  fn default() -> Self {
      Style {
          padding_inline: 0.,
          line_height: 1.5,
          font_size: 30.,
          font_family: "",
          font_style: FontStyle::Normal,
          font_weight: FontWeight::Bold,
          word_break: WordBreak::Normal,
          color: RGB(1., 1., 1.),
          // background_color: RGB(0., 0., 0.),
      }
  }
}
