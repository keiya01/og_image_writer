use super::font_trait::Font;
use super::Error;
use ab_glyph::{
    Font as AbFont, FontArc as AbFontArc, Glyph, GlyphId, OutlinedGlyph, ScaleFont as AbScaleFont,
};
use std::boxed::Box;

pub struct FontMetrics {
    pub height: f32,
    pub width: f32,
}

#[derive(Debug)]
pub struct FontArc(AbFontArc);

impl Font for FontArc {
    fn glyph_id(&self, ch: char) -> GlyphId {
        self.0.glyph_id(ch)
    }

    fn ascent(&self, scale: f32) -> f32 {
        self.0.as_scaled(scale).ascent()
    }

    fn descent(&self, scale: f32) -> f32 {
        self.0.as_scaled(scale).descent()
    }

    fn h_advance(&self, glyph_id: GlyphId, scale: f32) -> f32 {
        self.0.as_scaled(scale).h_advance(glyph_id)
    }

    fn kern(&self, first: GlyphId, second: GlyphId, scale: f32) -> f32 {
        self.0.as_scaled(scale).kern(first, second)
    }

    fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph> {
        self.0.as_scaled(scale).outline_glyph(glyph)
    }
}

pub const WHITESPACE_EM: f32 = 0.2;

pub fn create_font(data: Vec<u8>) -> Result<FontArc, Error> {
    match AbFontArc::try_from_vec(data) {
        Ok(font) => Ok(FontArc(font)),
        Err(_) => Err(Error::InvalidFontBytes),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct FontIndex(pub(super) usize);

#[derive(Debug, Clone, PartialEq)]
pub(super) enum FontIndexStore {
    Global(FontIndex),
    Parent(FontIndex),
    Child(FontIndex),
}

pub(super) struct FontStore(Vec<Box<dyn Font>>);

impl FontStore {
    pub(super) fn borrow_font(&self, idx: &FontIndex) -> &dyn Font {
        &*self.0[idx.0]
    }
}

mod font_context_store {
    use super::{Font, FontIndex, FontStore};
    use std::cell::RefCell;
    use std::rc::Rc;

    thread_local! {
        static FONT_CONTEXT_STORE: Rc<RefCell<FontStore>> = Rc::new(RefCell::new(FontStore(vec![])));
    }

    pub(super) fn get_mut() -> Rc<RefCell<FontStore>> {
        FONT_CONTEXT_STORE.with(|f| f.clone())
    }

    pub fn clear() {
        FONT_CONTEXT_STORE.with(|f| {
            let mut store = f.borrow_mut();
            store.0.clear();
        });
    }

    pub fn len() -> usize {
        FONT_CONTEXT_STORE.with(|f| {
            let store = f.borrow();
            store.0.len()
        })
    }

    pub(super) fn with<F, T>(idx: &FontIndex, f: F) -> T
    where
        F: FnOnce(&dyn Font) -> T,
    {
        let ctx = get_mut();
        let store = ctx.borrow();
        let font = store.borrow_font(idx);
        f(font)
    }
}

// This strut do not have nothing.
// But this struct provide operation for font_context_store local thread.
// If you want to use font_context_store, you must call method from FontContext.
// That is FontContext has role for access control for font_context_store.

/// You can specify global fallback font by using `FontContext::push`.
/// NOTE: FontContext will be shared with other instance.
#[derive(Default)]
pub struct FontContext;

impl FontContext {
    pub fn new() -> FontContext {
        FontContext
    }

    // TODO: optimize data structure for memory performance
    pub fn push(&mut self, data: Vec<u8>) -> Result<(), Error> {
        let store = font_context_store::get_mut();
        let mut store = store.borrow_mut();
        let font = create_font(data)?;
        store.0.push(Box::new(font));
        Ok(())
    }

    pub fn clear(&self) {
        // Clear global memory cache
        font_context_store::clear();
    }

    pub fn len(&self) -> usize {
        font_context_store::len()
    }

    pub fn is_empty(&self) -> bool {
        font_context_store::len() == 0
    }

    pub(super) fn select_font_family(&self, ch: char) -> Result<FontIndex, Error> {
        let store = font_context_store::get_mut();
        let font_list = &store.borrow().0;
        for (i, font) in font_list.iter().enumerate() {
            let has_font = match_font_family(ch, &**font);
            if has_font {
                return Ok(FontIndex(i));
            }
        }

        Err(Error::NotFoundSpecifiedFontFamily)
    }

    pub(super) fn with<F, T>(&self, idx: &FontIndex, f: F) -> T
    where
        F: FnOnce(&dyn Font) -> T,
    {
        font_context_store::with(idx, f)
    }
}

pub(super) fn match_font_family(ch: char, font: &dyn Font) -> bool {
    font.glyph_id(ch).0 != 0
}

pub(super) fn whitespace_width(size: f32) -> f32 {
    size * WHITESPACE_EM
}

#[cfg(test)]
pub(crate) mod test_utils {
    pub use super::*;
    use ab_glyph::{Outline, Point, PxScaleFactor, Rect};

    #[derive(Clone)]
    pub(crate) struct FontMock;

    impl Font for FontMock {
        fn glyph_id(&self, _ch: char) -> GlyphId {
            GlyphId(1)
        }

        fn ascent(&self, scale: f32) -> f32 {
            scale / 2.
        }

        fn descent(&self, scale: f32) -> f32 {
            -(scale / 2.)
        }

        fn h_advance(&self, _glyph_id: GlyphId, scale: f32) -> f32 {
            scale
        }

        fn kern(&self, _first: GlyphId, _second: GlyphId, _scale: f32) -> f32 {
            1.
        }

        fn outline_glyph(&self, glyph: Glyph, scale: f32) -> Option<OutlinedGlyph> {
            let point = Point { x: 0., y: 0. };
            let og = OutlinedGlyph::new(
                glyph,
                Outline {
                    bounds: Rect {
                        min: point,
                        max: point,
                    },
                    curves: Vec::new(),
                },
                PxScaleFactor {
                    horizontal: scale,
                    vertical: scale,
                },
            );
            Some(og)
        }
    }
}
