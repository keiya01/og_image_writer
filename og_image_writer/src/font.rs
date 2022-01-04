use super::Error;
use ab_glyph::{Font, FontArc};

pub const WHITESPACE_EM: f32 = 0.2;

pub fn create_font(data: Vec<u8>) -> Result<FontArc, Error> {
    match FontArc::try_from_vec(data) {
        Ok(font) => Ok(font),
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

pub(super) struct FontStore(Vec<FontArc>);

impl FontStore {
    pub(super) fn borrow_font(&self, idx: &FontIndex) -> &FontArc {
        &self.0[idx.0]
    }
}

mod font_context_store {
    use super::{FontIndex, FontStore};
    use ab_glyph::FontArc;
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
        F: FnOnce(&FontArc) -> T,
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
        store.0.push(font);
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
            let has_font = match_font_family(ch, font);
            if has_font {
                return Ok(FontIndex(i));
            }
        }

        Err(Error::NotFoundSpecifiedFontFamily)
    }

    pub(super) fn with<F, T>(&self, idx: &FontIndex, f: F) -> T
    where
        F: FnOnce(&FontArc) -> T,
    {
        font_context_store::with(idx, f)
    }
}

pub(super) fn match_font_family(ch: char, font: &FontArc) -> bool {
    font.glyph_id(ch).0 != 0
}

pub(super) fn whitespace_width(size: f32) -> f32 {
    size * WHITESPACE_EM
}
