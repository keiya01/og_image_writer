use std::iter::{Iterator, Peekable, Rev};
use std::str::CharIndices;

pub(super) fn is_newline(cur_char: char) -> bool {
    cur_char == '\n'
}

pub(super) fn is_rev_newline(cur_char: char) -> bool {
    cur_char == '\n'
}

pub(super) fn is_newline_as_whitespace(is_pre: bool, flag: &Option<CharFlags>) -> bool {
    !is_pre
        && flag
            .as_ref()
            .map(|f| matches!(f, CharFlags::Newline))
            .unwrap_or(false)
}

pub(crate) enum CharFlags {
    Newline,
}

// This is used as wrapper for CharIndices.
// If you want to consider newline or other character,
// you should wrap with RenderingCharIndices.
pub(crate) struct RenderingCharIndices<'a>(Peekable<CharIndices<'a>>);

impl<'a> RenderingCharIndices<'a> {
    pub(crate) fn from_str(s: &'a str) -> Self {
        let chars = s.char_indices().peekable();
        RenderingCharIndices(chars)
    }

    pub(crate) fn peek_char(&mut self) -> Option<char> {
        self.0.peek().map(|(_, c)| *c)
    }
}

impl<'a> Iterator for RenderingCharIndices<'a> {
    type Item = (Option<CharFlags>, usize, char, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let chars = &mut self.0;
        let v = chars.next();
        match v {
            Some((i, ch)) if is_newline(ch) => {
                Some((Some(CharFlags::Newline), i, ' ', ch.to_string().len()))
            }
            _ => v.map(|t| (None, t.0, t.1, t.1.to_string().len())),
        }
    }
}

// Reversed RenderingCharIndices.
pub(crate) struct RevRenderingCharIndices<'a>(Peekable<Rev<CharIndices<'a>>>);

impl<'a> RevRenderingCharIndices<'a> {
    pub(crate) fn from_str(s: &'a str) -> Self {
        let chars = s.char_indices().rev().peekable();
        RevRenderingCharIndices(chars)
    }

    pub(crate) fn peek_char(&mut self) -> Option<char> {
        self.0.peek().map(|(_, c)| *c)
    }
}

impl<'a> Iterator for RevRenderingCharIndices<'a> {
    type Item = (Option<CharFlags>, usize, char, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let chars = &mut self.0;
        let v = chars.next();
        match v {
            Some((i, ch)) if is_rev_newline(ch) => {
                Some((Some(CharFlags::Newline), i, ' ', ch.to_string().len()))
            }
            _ => v.map(|t| (None, t.0, t.1, t.1.to_string().len())),
        }
    }
}
