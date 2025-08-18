mod border;
#[cfg(test)]
mod tests;

pub use nu_ansi_term::{Color, Style as AnsiStyle};
pub use border::{BorderShape, BorderStyle};

use ansi_width::ansi_width;
use std::{cmp, io, fmt, borrow::{Borrow, Cow}};
use border::BorderChar;

#[derive(PartialEq, Eq, Debug)]
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
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Padding {
    pub chr: char,
    pub count: usize
}

impl Padding {
    pub const fn spaces(count: usize) -> Self {
        Self { chr: ' ', count }
    }

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

#[derive(Debug, Clone, Default)]
pub struct TermBox {
    pub border_style: BorderStyle,
    pub padding: Padding,
    pub lines: Vec<String>
}

impl TermBox {
    const SIDES: usize = 2;
    const MIN_LINE_LEN: usize = 3;

    /// Writes the box's text to the given [fmt::Write] implementor.
    pub fn write_to<T: fmt::Write>(&self, write: &mut T) -> fmt::Result {
        write!(write, "{}", self.to_string())
    }

    /// Writes the box's text to the file or other [io::Write] implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::TermBox;
    ///
    /// let box_ = TermBox::default();
    /// box_.print_to(&mut std::io::stderr()).expect("Printing box")
    /// ```
    pub fn print_to<T: io::Write>(&self, write: &mut T) -> io::Result<()> {
        write!(write, "{}", self.to_string())
    }

    /// Prints the box to [stdout](io::stdout).
    pub fn print(&self) {
        let _ = self.print_to(&mut io::stdout());
    }

    /// Converts the box to a [String].
    pub fn to_string(&self) -> String {
        let lines = self.lines.iter().map(CountedString::new).collect::<Vec<_>>();
        let longest_line = lines
            .iter()
            .max()
            .unwrap_or(&CountedString::EMPTY);

        let line_len = cmp::max(Self::MIN_LINE_LEN, line_len(longest_line, self.padding.count));

        // TODO estimate capacity needed for ANSI control sequences
        let mut buf = String::with_capacity((self.lines.len() + 2) * line_len);

        make_top_line(&mut buf, self.border_style, line_len);

        let edge_string = &self.border_style.get_edge_string();
        let pad_string = &self.padding.get_string();
        for line in lines.iter() {
            make_line(&mut buf, edge_string, pad_string, line, line_len)
        }

        make_bottom_line(&mut buf, self.border_style, line_len);

        buf
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

fn make_top_line(buf: &mut String, style: BorderStyle, len: usize) {
    make_top_or_bottom_line(buf, style, len, BorderChar::TopLeft, BorderChar::TopRight);
    buf.push('\n')
}

fn make_bottom_line(buf: &mut String, style: BorderStyle, len: usize) {
    make_top_or_bottom_line(buf, style, len, BorderChar::BotLeft, BorderChar::BotRight)
}

fn make_top_or_bottom_line(buf: &mut String, style: BorderStyle, len: usize, left: BorderChar, right: BorderChar) {
    let edge_char = style.shape.get_char(BorderChar::Edge);
    let char_len = edge_char.len();
    let mut tmp_buf = edge_char.repeat(len);

    // String.len() is in bytes
    tmp_buf.replace_range(0..char_len, style.shape.get_char(left));
    tmp_buf.replace_range((tmp_buf.len() - char_len).., style.shape.get_char(right));

    buf.push_str(&style.style.paint(tmp_buf).to_string());
}
