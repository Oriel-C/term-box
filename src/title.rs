use super::CountedString;

pub use cons::Title;

/// Represents the horizontal position of a title within the border of the [TermBox](super::TermBox).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TitlePosition {
    /// Tries to position the title in the center of the box's top/bottom border.
    ///
    /// If the parity (evenness/oddness) of the title's displayed length in characters does not
    /// match the parity of the box's width, it is impossible to properly center
    /// the title and it will be displayed slightly to the left of center.
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

    pub(crate) fn len_bytes(&self) -> usize { self.text().len() }

    pub(crate) fn left_pad_len(&self, total_len: usize, dist_from_corner: usize) -> usize {
        let width = self.text.width;
        if width == 0 {
            return total_len / 2;
        }

        #[cfg(test)]
        assert!(total_len > width);

        match self.pos {
            TitlePosition::Left => dist_from_corner,
            TitlePosition::Right => total_len - width - (dist_from_corner + 1), // +1: must include corner
            TitlePosition::Centered => (total_len / 2) - (width / 2) - 1 // -1: make 0-base
        }
    }
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
