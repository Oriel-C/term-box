//! [Line] type and the [lines] macro.

use std::cmp;
use std::borrow::{Borrow, Cow};
use ansi_width::ansi_width;

/// Creates a vector of [Lines](Line) for a [TermBox](super::TermBox).
///
/// All arguments must implement [ToString] or otherwise have a `to_string` method.
/// 
/// # Examples
///
/// ```
/// use term_box::{TermBox, lines, AnsiStyle};
///
/// let box_ = TermBox {
///     lines: lines![
///         4,
///         "lines of",
///         AnsiStyle::new().bold().paint("styled"),
///         String::from("text")
///     ],
///     ..TermBox::default()
/// };
///
/// let output = format!("
/// ┌────────┐
/// │4       │
/// │lines of│
/// │{lin3}  │
/// │text    │
/// └────────┘
/// ", lin3 = AnsiStyle::new().bold().paint("styled"));
///
/// assert_eq!(box_.into_string(), output.trim());
/// ```
#[macro_export]
macro_rules! lines {
    ($($lines:expr),*) => {
        vec![ $($lines.to_string()),* ]
    };
}

pub use lines;

/// A line of text in a [TermBox](super::TermBox).
///
/// Currently, this is just an alias for a [String]. However,
/// it may change to be a unique struct in the future. Care will be taken
/// to not break (most) old code ([Line::new] and [Line::from] should still work).
pub type Line = String;

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
