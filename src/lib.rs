mod border;
#[cfg(test)]
mod tests;
pub mod title;

pub use nu_ansi_term::{Color, Style as AnsiStyle};
pub use border::{BorderShape, BorderStyle};
pub use title::*;

use ansi_width::ansi_width;
use std::{cmp, io, fmt, borrow::{Borrow, Cow}};
use border::BorderChar;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
struct CountedString<'a> {
    str: Cow<'a, str>,
    width: usize
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
    fn new(string: impl Into<Cow<'a, str>>) -> Self {
        let str = string.into();
        let width = ansi_width(str.borrow());
        Self { str, width }
    }

    fn str(&'a self) -> &'a str {
        self.str.borrow()
    }
}

impl CountedString<'static> {
    const EMPTY: Self = Self { str: Cow::Borrowed(""), width: 0 };

    fn owned(string: String) -> Self {
        Self { width: ansi_width(&string), str: Cow::Owned(string) }
    }
}

/// Represents the padding between the edge of the [TermBox] and the text
/// it contains. By default, no padding is used.
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

    /// Creats a new [Padding] that will not actually pad text.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::Padding;
    /// assert_eq!(Padding::default(), Padding::none());
    /// ```
    pub const fn none() -> Self { Self::new('\0', 0) }

    /// Creates a new [Padding] that pads with the given character and number of spaces.
    ///
    /// If the passed [char] is a tab character, it will be replaced with 8 spaces to
    /// prevent misaligned edges.
    pub const fn new(chr: char, count: usize) -> Self {
        match chr {
            '\t' => Self::spaces(count * 8),
            _    => Self { chr, count }
        }
    }

    /// Creates a new [Padding] that pads with the given number of spaces.
    pub const fn spaces(count: usize) -> Self {
        Self { chr: ' ', count }
    }

    pub const fn len_utf8(self) -> usize {
        self.chr.len_utf8() * self.count
    }

    /// Returns the [char] used for padding.
    pub const fn chr(self) -> char { self.chr }

    /// Returns the number of times the [chr](Padding::chr) will be
    /// repeated in padding.
    pub const fn count(self) -> usize { self.count }

    fn get_string(self) -> CountedString<'static> {
        match self.count {
            0 => CountedString::EMPTY,
            n => CountedString {
                str: Cow::Owned(String::from(self.chr).repeat(n)),
                width: n
            }
        }
    }
}

/// Represents text in a box that can be displayed in a terminal or other output.
#[derive(Debug, Clone, Default)]
pub struct TermBox {
    /// [BorderStyle] describing how the edges of the box should be styled.
    pub border_style: BorderStyle,
    /// [Padding] describing how the text should be padded.
    pub padding: Padding,
    /// [Titles] for the box.
    pub titles: Titles,
    /// Lines of text to display in the box.
    pub lines: Vec<String>
}

impl TermBox {
    const SIDES: usize = 2;
    const MIN_LINE_LEN: usize = 3;

    /// Appends an additional line to the box's contents.
    pub fn append(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    /// Appends an additional line to the owned box's contents and
    /// returns the box.
    pub fn append_with(mut self, line: impl ToString) -> Self {
        self.append(line);
        self
    }

    /// Writes the box text to the given [fmt::Write] implementor WITHOUT a final newline.
    pub fn write_to<T: fmt::Write>(self, write: &mut T) -> fmt::Result {
        write!(write, "{}", self.into_string())
    }

    /// Writes the box to the file or other [io::Write] implementor WITH a final newline.\
    /// If the implementor is not connected to a terminal, ANSI styles may not display
    /// properly.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::TermBox;
    ///
    /// let box_ = TermBox::default();
    /// // Print the box to stderr:
    /// box_.print_to(&mut std::io::stderr()).expect("could not print box to stderr")
    /// ```
    pub fn print_to<T: io::Write>(self, write: &mut T) -> io::Result<()> {
        writeln!(write, "{}", self.into_string())
    }

    /// Prints the box to [stdout](io::stdout) with a final newline.
    pub fn print(self) {
        let _ = self.print_to(&mut io::stdout());
    }

    /// Converts the box to a [String].
    pub fn into_string(self) -> String {
        let mut lines = Vec::with_capacity(self.lines.len());
        let mut longest_line: &CountedString = cmp::max(&self.titles.top.text, &self.titles.bottom.text);
        if let Some(longest_idx) = self.count_and_find_longest(&mut lines) {
            longest_line = cmp::max(longest_line, &lines[longest_idx]);
        }

        let line_len = cmp::max(Self::MIN_LINE_LEN, line_len(longest_line, self.padding.count));

        // TODO estimate capacity needed for ANSI control sequences
        let mut buf = String::with_capacity((lines.len() + 2) * line_len);

        make_top_line(&mut buf, &self, line_len);

        let edge_string = &self.border_style.get_edge_string();
        let pad_string = &self.padding.get_string();
        for line in lines.iter() {
            make_line(&mut buf, edge_string, pad_string, line, line_len)
        }

        make_bottom_line(&mut buf, &self, line_len);

        buf
    }

    fn count_and_find_longest<'a>(&'a self, lines: &mut Vec<CountedString<'a>>) -> Option<usize> {
        let mut max_idx = None;

        for (idx, line) in self.lines.iter().map(CountedString::new).enumerate() {
            match max_idx {
                Some(max) if line > lines[max] => max_idx = Some(idx),
                None => max_idx = Some(idx),
                _ => {}
            }
            lines.push(line)
        }

        max_idx
    }
}

fn line_len(line: &CountedString, padding: usize) -> usize {
    line.width + TermBox::SIDES + (TermBox::SIDES * padding)
}

fn make_line(buf: &mut String, edge_string: &str, pad_string: &CountedString, text: &CountedString, min_len: usize) {
    buf.push_str(edge_string);
    buf.push_str(pad_string.str());
    buf.push_str(text.str());
    
    let diff = min_len - line_len(text, pad_string.width);
    if diff > 0 {
        buf.push_str(&str::repeat(" ", diff))
    }

    buf.push_str(pad_string.str());
    buf.push_str(edge_string);
    buf.push('\n')
}

struct HorizLineArgs<'a> {
    len: usize,
    style: BorderStyle,
    title: &'a Title,
    left: BorderChar,
    right: BorderChar
}

fn make_top_line(buf: &mut String, tbox: &TermBox, len: usize) {
    make_top_or_bottom_line(buf, HorizLineArgs {
        len,
        style: tbox.border_style, title: &tbox.titles.top,
        left: BorderChar::TopLeft, right: BorderChar::TopRight
    });
    buf.push('\n')
}

fn make_bottom_line(buf: &mut String, tbox: &TermBox, len: usize) {
    make_top_or_bottom_line(buf, HorizLineArgs {
        len,
        style: tbox.border_style, title: &tbox.titles.bottom,
        left: BorderChar::BotLeft, right: BorderChar::BotRight
    })
}

const DEFAULT_DIST_FROM_CORNER: usize = 1;

fn make_top_or_bottom_line(buf: &mut String, args: HorizLineArgs) {
    let style = args.style;
    let shape = style.shape;
    let edge_char = shape.get_char(BorderChar::Edge);
    // String.len() is in bytes
    let mut tmp_buf = alloc_title_buf(&args);
    tmp_buf += shape.get_char(args.left);

    let right_char = shape.get_char(args.right);
    if !args.title.is_empty() {
        tmp_buf = ins_title(tmp_buf, edge_char, right_char, &args);
    } else {
        tmp_buf += &(edge_char.repeat(args.len - TermBox::SIDES) + right_char);
    }

    // Works in all cases except a styled right title, which would be fairly complicated
    // for something not very worth covering for
    // let actual = tmp_buf.len();
    // assert!(actual == init_cap, "{actual} != {init_cap}");

    if style.ansi.is_plain() {
        buf.push_str(&tmp_buf);
    } else {
        buf.push_str(&style.ansi.paint(tmp_buf).to_string());
    }
}

fn alloc_title_buf(args: &HorizLineArgs) -> String {
    let mut cap = BorderChar::NUM_BYTES * (args.len - args.title.text.width);
    cap += args.title.len_bytes();
    String::with_capacity(cap)
}

fn ins_title(mut buf: String, edge_char: &str, right_char: &str, args: &HorizLineArgs) -> String {
    let title = args.title;
    let left_pad_len = title.left_pad_len(args.len, DEFAULT_DIST_FROM_CORNER);
    buf += &edge_char.repeat(left_pad_len);
    buf += title.text.str();

    let right_pad_len = args.len - title.text.width - left_pad_len - TermBox::SIDES; // -2: corners
    // titles may reset the style, so apply it again
    buf += &args.style.ansi.paint(edge_char.repeat(right_pad_len) + right_char).to_string();
    buf
}
