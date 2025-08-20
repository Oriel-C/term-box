use super::CountedString;

pub use cons::Title;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TitlePosition {
    Centered,
    #[default]
    Left,
    Right
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Title {
    pub(crate) text: CountedString<'static>,
    pub(crate) pos: TitlePosition
}

/// A title displayed in the border of the box itself.
///
/// Construct with [Title::empty] or the [Title](cons::Title) function.
impl Title {
    pub fn empty() -> Self { Self::default() }

    pub fn is_empty(&self) -> bool {
        // len, not width: may be used for control characters
        self.text.str().len() == 0
    }

    pub(crate) fn len_bytes(&self) -> usize {
        self.text.str.len()
    }

    pub(crate) fn left_pad_len(&self, total_len: usize, dist_from_corner: usize) -> usize {
        if self.text.width == 0 {
            return total_len / 2;
        }

        assert!(total_len > self.text.width);

        match self.pos {
            TitlePosition::Left => dist_from_corner,
            TitlePosition::Right => total_len - self.text.width - (dist_from_corner + 1), // +1: must include corner
            TitlePosition::Centered => (total_len / 2) - (self.text.width / 2) - 1 // -1: make 0-base
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Titles {
    pub top: Title,
    pub bottom: Title
}

impl Titles {
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
