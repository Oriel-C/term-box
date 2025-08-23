#[allow(unused_imports)] // Used extensively in documentation, makes writing it easier.
use super::TermBox;
use super::CountedString;

/// Represents the padding between the edge of the [TermBox] and the text
/// it contains.
///
/// Padding appears between the horizontal edges of a [TermBox] and
/// the lines of text within. To vertically pad a [TermBox], add blank lines
/// to the start and end of the [lines](TermBox::lines) vector.
///
/// By default, boxes have no padding ([Padding::none]).
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Padding {
    /// The [char] used to provide the padding (usually spaces or tabs).
    chr: char,
    /// The number of the [chr](Padding::chr) that should be used for the padding.
    count: usize
}

impl Padding {
    /// Pad the edges of a [TermBox's](TermBox) text by one space.
    pub const ONE_SPACE: Padding = Self::spaces(1);

    /// Creates a new [Padding] that will not actually pad text.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::Padding;
    /// assert_eq!(Padding::default(), Padding::none());
    /// assert_eq!("", Padding::none().into_string());
    /// ```
    pub const fn none() -> Self { Self::new('\0', 0) }

    /// Creates a new [Padding] that pads with the given character and number of spaces.
    ///
    /// If the passed [char] is a tab character, it will be replaced with 8 spaces to
    /// prevent misaligned edges. Other whitespace characters are not accounted for
    /// and may cause formatting issues.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::Padding;
    ///
    /// let padding = Padding::new('-', 2);
    /// assert_eq!("--", padding.into_string());
    /// ```
    ///
    /// Tab = spaces:
    ///
    /// ```
    /// use term_box::Padding;
    ///
    /// assert_eq!(Padding::new('\t', 1), Padding::new(' ', 8));
    /// ```
    pub const fn new(chr: char, count: usize) -> Self {
        match chr {
            '\t' => Self::spaces(count * 8),
            _    => Self { chr, count }
        }
    }

    /// Creates a new [Padding] that pads with the given number of spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::Padding;
    ///
    /// let padding = Padding::spaces(1);
    /// assert_eq!(padding, Padding::ONE_SPACE);
    /// assert_eq!(" ", padding.into_string());
    /// ```
    pub const fn spaces(count: usize) -> Self {
        Self { chr: ' ', count }
    }

    /// Gets the length of the padding in bytes once converted into a string.
    pub const fn len_utf8(self) -> usize {
        self.chr.len_utf8() * self.count
    }

    /// Returns the [char] used for padding.
    pub const fn chr(self) -> char { self.chr }

    /// Returns the number of times the [chr](Padding::chr) will be
    /// repeated in padding.
    pub const fn count(self) -> usize { self.count }

    /// Converts the padding into a string and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::Padding;
    ///
    /// let padding = Padding::new('a', 3);
    /// assert_eq!("aaa", padding.into_string());
    /// ```
    pub fn into_string(self) -> String {
        String::from(self.chr).repeat(self.count)
    }

    pub(super) fn into_counted_string(self) -> CountedString<'static> {
        match self.count {
            0 => CountedString::EMPTY,
            n => CountedString::counted(self.into_string(), n)
        }
    }
}
