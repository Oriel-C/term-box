use std::{borrow::{Borrow, Cow}, cmp};
use ansi_width::ansi_width;

#[macro_export]
macro_rules! lines {
    ($($lines:expr),*) => {
        vec![ $($lines.to_string()),* ]
    };
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub(crate) struct CountedString<'a> {
    str: Cow<'a, str>,
    pub(crate) width: usize
}

impl cmp::PartialOrd for CountedString<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for CountedString<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.width.cmp(&other.width)
    }
}

impl<'a> CountedString<'a> {
    pub fn new(string: impl Into<Cow<'a, str>>) -> Self {
        let str = string.into();
        let width = ansi_width(str.borrow());
        Self { str, width }
    }

    pub fn str(&'a self) -> &'a str {
        self.str.borrow()
    }
}

impl CountedString<'static> {
    pub const EMPTY: Self = Self { str: Cow::Borrowed(""), width: 0 };

    pub fn counted(string: String, width: usize) -> Self {
        Self { str: Cow::Owned(string), width }
    }

    pub fn owned(string: String) -> Self {
        let width = ansi_width(&string);
        Self::counted(string, width)
    }
}


