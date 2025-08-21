use super::{CountedString, DEFAULT_DIST_FROM_CORNER, TermBox};

pub use cons::Title;

/// Represents the horizontal position of a title within the border of the [TermBox](super::TermBox).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TitlePosition {
    /// Tries to position the title in the center of the box's top/bottom border.
    ///
    /// If the parity (evenness/oddness) of the title's displayed length in characters does not
    /// match the parity of the box's width, it is impossible to properly center
    /// the title and it will be displayed slightly to the left or right of center.
    Centered,
    /// Tries to position the title on the lefthand side of the box's top/bottom border,
    /// one character away from the corner.
    #[default]
    Left,
    /// Tries to position the title on the righthand side of the box's top/bottom border,
    /// one character away from the corner.
    Right
}

/// A title displayed in the border of the box itself.
///
/// Construct with [Title::empty] or the [Title](cons::Title) function.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Title {
    pub(crate) text: CountedString<'static>,
    pub(crate) pos: TitlePosition
}

impl Title {
    /// Creates a new, empty title.
    pub fn empty() -> Self { Self::default() }

    /// Returns whether the title's text is empty.
    pub fn is_empty(&self) -> bool {
        // len, not width: may be used for control characters
        self.text.str().is_empty()
    }

    /// Returns the title's text.
    pub fn text(&self) -> &str { self.text.str() }

    /// Returns the title's position.
    pub fn pos(&self) -> TitlePosition { self.pos }

    pub(crate) fn width(&self) -> usize { self.text.width }

    pub(crate) fn len_bytes(&self) -> usize { self.text().len() }

    pub(crate) fn left_pad_len(&self, total_len: usize) -> usize {
        let width = self.width();
        if let Some(pad_len) = special_pad_len(width, total_len) {
            return pad_len
        }

        match self.pos {
            TitlePosition::Left => DEFAULT_DIST_FROM_CORNER,
            TitlePosition::Right => opposite_side_pad_len(width, total_len),
            TitlePosition::Centered => {
                center_pad_len(width, total_len, 0)
            }
        }
    }

    pub(crate) fn right_pad_len(&self, total_len: usize) -> usize {
        let width = self.width();
        if let Some(pad_len) = special_pad_len(width, total_len) {
            return pad_len
        }

        match self.pos {
            TitlePosition::Right => DEFAULT_DIST_FROM_CORNER,
            TitlePosition::Left => opposite_side_pad_len(width, total_len),
            TitlePosition::Centered => {
                center_pad_len(width, total_len, 1)
            }
        }
    }
}

fn special_pad_len(width: usize, total_len: usize) -> Option<usize> {
    if width == 0 {
        return Some(total_len / 2);
    }

    #[cfg(test)]
    assert!(total_len > width);

    let diff = (total_len - TermBox::SIDES) - width;
    match diff {
        0 => Some(0),
        _ => None
    }
}

fn opposite_side_pad_len(width: usize, total_len: usize) -> usize {
    total_len - width - DEFAULT_DIST_FROM_CORNER - TermBox::SIDES
}

fn center_pad_len(width: usize, total_len: usize, parity_diff_mod: usize) -> usize {
    const ODD: usize = 1;
    const EVEN: usize = 0;

    let res = (total_len / 2) - (width / 2) - 1;
    let total_parity = total_len & 1;
    if (width & 1) != total_parity {
        return match total_parity {
            ODD => res + parity_diff_mod,
            EVEN => res - parity_diff_mod,
            _ => panic!("Impossible condition")
        }
    }

    res
}

/// The titles for a [TermBox](super::TermBox). Each [Title] is placed
/// independently.
///
/// A term box may have up to two titles: one at the top, one at the bottom.
/// Titles are placed inside the border of the box.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Titles {
    /// The title to display at the top of the box.
    ///
    /// Use [Title::empty] for no title.
    pub top: Title,
    /// The title to display at the bottom of the box.
    ///
    /// Use [Title::empty] for no title.
    pub bottom: Title
}

impl Titles {
    /// Constructs [Titles] such that no titles will be displayed in the box.
    pub fn none() -> Self { Self::default() }
}

mod cons {
    use super::*;

    /// Constructs a new [Title](super::Title).
    #[allow(non_snake_case)]
    pub fn Title(text: impl ToString, pos: TitlePosition) -> Title {
        Title {
            text: CountedString::owned(text.to_string()),
            pos
        }
    }
}
