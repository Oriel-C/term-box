use std::{fmt, cmp, io};
use super::*;

mod format;

pub(crate) use format::DEFAULT_DIST_FROM_CORNER;

/// Represents text in a box that can be displayed in a terminal or other output.
///
/// See the [module-level documentation](index.html) for more details.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TermBox {
    /// [BorderStyle] describing how the edges of the box should be styled.
    pub border_style: BorderStyle,
    /// [Padding] describing how the text should be padded.
    pub padding: Padding,
    /// [Titles] for the box.
    pub titles: Titles,
    /// Lines of text to display in the box.
    pub lines: Vec<Line>
}

impl TermBox {
    pub(crate) const SIDES: usize = 2;
    const MIN_LINE_LEN: usize = 3;

    /// Appends an additional line to the box's contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::{lines, Line, TermBox};
    /// const WORLD: &str = "world!";
    ///
    /// let mut lines = lines![ "hello" ];
    /// let mut append_box = TermBox { lines: lines.clone(), ..TermBox::default() };
    ///
    /// lines.push(Line::from(WORLD));
    /// append_box.append(WORLD);
    ///
    /// let push_box = TermBox { lines, ..TermBox::default() };
    /// assert_eq!(append_box, push_box);
    /// ```
    pub fn append(&mut self, line: impl ToString) {
        self.lines.push(line.to_string());
    }

    /// Appends an additional line to the owned box's contents an returns the box.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::{lines, Line, TermBox};
    /// const WORLD: &str = "world!";
    ///
    /// let mut lines = lines![ "hello" ];
    /// let append_box = TermBox { lines: lines.clone(), ..TermBox::default() };
    ///
    /// lines.push(Line::from(WORLD));
    ///
    /// let push_box = TermBox { lines, ..TermBox::default() };
    /// assert_eq!(append_box.append_with(WORLD), push_box);
    /// ```
    pub fn append_with(mut self, line: impl ToString) -> Self {
        self.append(line);
        self
    }

    /// Writes the box text to the given [fmt::Write] implementor WITHOUT a final newline.
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::TermBox;
    ///
    /// let empty_box = TermBox::default();
    /// let mut out_str = String::new();
    ///
    /// empty_box.write_to(&mut out_str);
    /// assert_eq!(out_str, "┌─┐\n└─┘");
    /// assert_ne!(out_str, "┌─┐\n└─┘\n");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use term_box::TermBox;
    ///
    /// let empty_box = TermBox::default();
    ///
    /// // Same output:
    /// empty_box.clone().print();
    /// println!("{}", empty_box.into_string());
    /// ```
    pub fn print(self) {
        let _ = self.print_to(&mut io::stdout());
    }

    /// Converts the box to a [String] for display in the terminal.
    pub fn into_string(self) -> String {
        let mut lines = Vec::with_capacity(self.lines.len());
        let mut longest_line: &CountedString = cmp::max(&self.titles.top.text, &self.titles.bottom.text);
        if let Some(longest_idx) = self.map_to_counts_and_find_longest(&mut lines) {
            longest_line = cmp::max(longest_line, &lines[longest_idx]);
        }

        let line_len = cmp::max(Self::MIN_LINE_LEN, format::line_len(longest_line, self.padding.count()));
        let mut buf = String::with_capacity((lines.len() + 2) * line_len);

        format::make_top_line(&mut buf, &self, line_len);

        let edge_string = &self.border_style.get_edge_string();
        let pad_string = &self.padding.into_counted_string();
        for line in lines.iter() {
            format::make_line(&mut buf, edge_string, pad_string, line, line_len)
        }

        format::make_bottom_line(&mut buf, &self, line_len);

        buf
    }

    fn map_to_counts_and_find_longest<'a>(&'a self, lines: &mut Vec<CountedString<'a>>) -> Option<usize> {
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
