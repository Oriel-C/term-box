pub use nu_ansi_term::{Color, Style as AnsiStyle};
use ansi_width::ansi_width;
mod border;

use std::{cmp, io, fmt};
pub use border::{BorderShape, BorderStyle};
use border::BorderChar;

#[derive(PartialEq, Eq)]
struct CountedString<'a> {
    str: &'a str,
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
    fn new(string: &'a impl AsRef<str>) -> Self {
        let str = string.as_ref();
        CountedString { str, width: ansi_width(str) }
    }
}

impl CountedString<'static> {
    const EMPTY: Self = CountedString { str: "", width: 0 };
}

#[derive(Debug, Clone, Default)]
pub struct TermBox {
    pub border_style: BorderStyle,
    pub lines: Vec<String>
}

impl TermBox {
    const MIN_LINE_LEN: usize = (BorderChar::LEN_BYTES * 2) + 1; // 2 border chars, '\n'
    const MIN_EDGE_LEN: usize = (BorderChar::LEN_BYTES * 3) + 1; // 3 border chars (corners and edge) + '\n'

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

        let line_len = cmp::max(3, longest_line.width + 2); // +3: '|', '|', skip '\n'
        let line_len_bytes = cmp::max(Self::MIN_EDGE_LEN, longest_line.width + Self::MIN_LINE_LEN);

        // TODO estimate capacity needed for ANSI control sequences
        let mut buf = String::with_capacity((self.lines.len() + 2) * line_len_bytes);

        make_top_line(&mut buf, self.border_style, line_len);

        let edge_string = self.border_style.get_edge_string();
        for line in lines.iter() {
            make_line(&mut buf, &edge_string, line, line_len_bytes)
        }

        make_bottom_line(&mut buf, self.border_style, line_len);

        buf
    }
}

fn make_line(buf: &mut String, edge_string: &str, text: &CountedString, min_len: usize) {
    buf.push_str(edge_string);
    buf.push_str(text.str);
    
    let diff = min_len - (text.width + (2 * BorderChar::LEN_BYTES) + 1);
    if diff > 0 {
        buf.push_str(&str::repeat(" ", diff))
    }

    buf.push_str(edge_string);
    buf.push('\n');
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

#[cfg(test)]
mod tests {
    use super::*;
    use derive_new::new;
    use std::fs;

    macro_rules! strings {
        ($($strs:expr),*) => {
            vec![ $($strs.to_string()),* ]
        }
    }

    macro_rules! assert_okay {
        ($expr:expr $(, $name:literal)?) => {
            match $expr {
                Ok(val) => val,
                Err(err) => panic!(concat!("Assertion ", $("'", $name, "' ",)? "failed: {:?}"), err)
            }
        };
    }

    macro_rules! template_name {
        ($name:literal) => { concat!("test-input/", $name, ".txt") };
    }

    macro_rules! assert_matches_template {
        ($box:expr, $template:literal) => {{
            const TEMPLATE_PATH: &str = template_name!($template);
            let template = assert_okay!(fs::read_to_string(TEMPLATE_PATH), "template exists");
            assert_eq!($box, template);
        }};
    }

    #[allow(dead_code)]
    macro_rules! init_template {
        ($box:expr, $template:expr) => {{
            const TEMPLATE_PATH: &str = template_name!($template);
            let _ = fs::write(TEMPLATE_PATH, $box).unwrap();
            assert!(false, "Test not yet finalized. Check {} contents and change to proper assertion.", TEMPLATE_PATH)
        }};
    }

    #[derive(Debug, new)]
    #[allow(dead_code)]
    struct LineLenErr {
        expected: usize,
        found: usize,
        at: usize
    }

    fn lines_same_len(string: &str) -> Result<usize, LineLenErr> {
        string.split('\n')
            .into_iter()
            .enumerate()
            .try_fold(0, |len, (idx, next)| {
                let next_len = ansi_width(next);
                match len {
                    0 => Ok(next_len),
                    _ => if len == next_len { Ok(len) } else { Err(LineLenErr::new(len, next_len, idx)) }
                }
            })
    }

    #[test]
    fn empty() {
        let box_single = TermBox::default().to_string();
        let box_double = TermBox { border_style: BorderStyle::new_double(), lines: Vec::new() }.to_string();
        assert_eq!(box_single, "┌─┐\n└─┘");
        assert_eq!(box_double, "╔═╗\n╚═╝");
    }

    #[test]
    fn empty_styled() {
        let box_ = TermBox {
            border_style: BorderStyle::new_double().with_style(Color::Purple),
            lines: Vec::new()
        }.to_string();

        assert_okay!(lines_same_len(&box_));
        assert_matches_template!(box_, "empty-styled");
    }


    #[test]
    fn unstyled() {
        let box_ = TermBox {
            border_style: BorderStyle::default(),
            lines: strings![
                "a",
                "few",
                "lines",
                "for testing"
            ]
        }.to_string();

        assert_okay!(lines_same_len(&box_));
        assert_eq!(box_, "┌───────────┐\n│a          │\n│few        │\n│lines      │\n│for testing│\n└───────────┘");
    }

    #[test]
    fn unstyled_with_ansi_text() {
        let box_ = TermBox {
            border_style: BorderStyle::default(),
            lines: strings![
                "uncolored",
                Color::Red.paint("colored!!"),
                AnsiStyle::new().bold().paint("bolded"),
                Color::Blue.bold().paint("both")
            ]
        }.to_string();

        assert_okay!(lines_same_len(&box_));
        assert_matches_template!(box_, "unstyled-with-ansi-text");
    }

    #[test]
    fn styled() {
        let box_ = TermBox {
            border_style: BorderStyle::new_single().with_style(Color::LightPurple.bold()),
            lines: strings![
                "some",
                "cool",
                "text"
            ]
        }.to_string();

        assert_okay!(lines_same_len(&box_));
        assert_matches_template!(box_, "styled");
    }

    #[test]
    fn styled_with_ansi_text() {
        let box_ = TermBox {
            border_style: BorderStyle::new_double().with_style(Color::Black.italic()),
            lines: strings![
                "uncolored",
                Color::Red.paint("colored!!"),
                AnsiStyle::new().bold().paint("bolded"),
                Color::Blue.bold().paint("both")
            ]
        }.to_string();

        assert_okay!(lines_same_len(&box_));
        assert_matches_template!(box_, "styled-with-ansi-text");
    }
}
