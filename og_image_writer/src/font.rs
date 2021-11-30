use super::Error;
use std::cell::RefCell;
use std::rc::Rc;

pub use rusttype::{Font, IntoGlyphId};

pub fn create_font<'a>(data: Vec<u8>) -> Result<Font<'a>, Error> {
    match Font::try_from_vec(data) {
        Some(font) => Ok(font),
        None => Err(Error::InvalidFontBytes),
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

pub(super) struct FontStore(Vec<Font<'static>>);

impl FontStore {
    pub(super) fn borrow_font(&self, idx: &FontIndex) -> &Font<'static> {
        &self.0[idx.0]
    }
}

mod font_context_store {
    use super::FontStore;
    use std::cell::RefCell;
    use std::rc::Rc;

    thread_local! {
        static FONT_CONTEXT_STORE: Rc<RefCell<FontStore>> = Rc::new(RefCell::new(FontStore(vec![])));
    }

    pub(super) fn get_mut() -> Rc<RefCell<FontStore>> {
        FONT_CONTEXT_STORE.with(|f| f.clone())
    }
}

// This strut do not have nothing.
// But this struct provide operation for font_context_store local thread.
// If you want to use font_context_store, you must call method from FontContext.
// That is FontContext has role for access control for font_context_store.
pub struct FontContext;

impl FontContext {
    pub(super) fn new() -> FontContext {
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

    pub(super) fn borrow_font_store(&self) -> Rc<RefCell<FontStore>> {
        font_context_store::get_mut()
    }
}

pub(super) fn match_font_family(ch: char, font: &Font) -> bool {
    ch.into_glyph_id(font).0 != 0
}
