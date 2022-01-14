use super::font_context::FontIndexStore;
use std::ops::Range;

#[derive(Debug)]
pub(crate) struct Glyph {
    pub(super) range: Range<usize>,
    // If font_index is -1, font_index indicates font that element has.
    pub(super) font_index_store: FontIndexStore,
}

impl Glyph {
    pub(crate) fn new(range: Range<usize>, font_index_store: FontIndexStore) -> Glyph {
        Glyph {
            range,
            font_index_store,
        }
    }
}
