pub use nu_ansi_term::{Color, Style as AnsiStyle};
use ansi_width::ansi_width;
use std::cmp;

#[repr(usize)]
#[derive(Debug)]
enum BorderChar {
    TopLeft = 0,
    TopRight = 1,
    Side = 2,
    BotLeft = 3,
    BotRight = 4,
    Edge = 5
}

impl BorderChar {
    const LEN_BYTES: usize = BorderShape::SINGLE_SHAPES[0].len();
}

#[derive(Debug, Clone, Copy, Default)]
pub enum BorderShape {
    #[default]
    Single,
    Double
}

impl BorderShape {
    const SINGLE_SHAPES: [&str; 6] = [ "┌", "┐", "│", "└", "┘", "─" ];
    const DOUBLE_SHAPES: [&str; 6] = [ "╔", "╗", "║", "╚", "╝", "═" ];

    fn get_char(self, char: BorderChar) -> &'static str {
        match self {
            Self::Single => Self::SINGLE_SHAPES[char as usize],
            Self::Double => Self::DOUBLE_SHAPES[char as usize]
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BorderStyle {
    shape: BorderShape,
    style: AnsiStyle
}

impl BorderStyle {
    fn get_edge_string(&self) -> String {
        let base = self.shape.get_char(BorderChar::Side);
        self.style.paint(base).to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Box {
    border_style: BorderStyle,
    lines: Vec<String>
}

impl Box {
    const MIN_LINE_LEN: usize = (BorderChar::LEN_BYTES * 2) + 1; // 2 border chars, '\n'
    const MIN_EDGE_LEN: usize = (BorderChar::LEN_BYTES * 3) + 1; // 3 border chars (corners and edge) + '\n'

    pub fn print(&self) {
        print!("{}", self.to_string())
    }

    pub fn to_string(&self) -> String {
        let empty = String::from("");
        let longest_line = self.lines
            .iter()
            .max_by(|a, b| ansi_width(a).cmp(&ansi_width(b)))
            .unwrap_or(&empty);

        let line_len = cmp::max(3, longest_line.chars().count() + 2); // +3: '|', '|', skip '\n'
        let line_len_bytes = cmp::max(Self::MIN_EDGE_LEN, longest_line.len() + Self::MIN_LINE_LEN);

        // TODO estimate capacity needed for ANSI control sequences
        let mut buf = String::with_capacity((self.lines.len() + 2) * line_len_bytes);

        make_top_line(&mut buf, self.border_style, line_len);

        let edge_string = self.border_style.get_edge_string();
        for line in self.lines.iter() {
            make_line(&mut buf, &edge_string, line, line_len_bytes)
        }

        make_bottom_line(&mut buf, self.border_style, line_len);

        buf
    }
}

fn make_line(buf: &mut String, edge_string: &str, text: &str, min_len_bytes: usize) {
    buf.push_str(edge_string);
    buf.push_str(text);
    
    let diff = min_len_bytes - (text.len() + (2 * BorderChar::LEN_BYTES) + 1);
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

    macro_rules! strings {
        ($($strs:literal),*) => {
            vec![ $(String::from($strs)),* ]
        }
    }

    #[derive(Debug, new)]
    #[allow(dead_code)]
    struct LineLenErr {
        expected: usize,
        found: usize,
        at_index: usize
    }

    fn lines_same_len(string: &str) -> Result<usize, LineLenErr> {
        string.split('\n')
            .into_iter()
            .enumerate()
            .try_fold(0, |len, (idx, next)| {
                let next_len = next.chars().count();
                match len {
                    0 => Ok(next_len),
                    _ => if len == next_len { Ok(len) } else { Err(LineLenErr::new(len, next_len, idx)) }
                }
            })
    }

    #[test]
    fn empty() {
        let box_ = Box::default().to_string();
        assert_eq!(box_, "┌─┐\n└─┘");
    }

    #[test]
    fn unstyled() {
        let box_ = Box {
            border_style: BorderStyle::default(),
            lines: strings![
                "a",
                "few",
                "lines",
                "for testing"
            ]
        }.to_string();

        let assertion = lines_same_len(&box_);
        assert!(assertion.is_ok(), "{assertion:?}");
        assert_eq!(box_, "┌───────────┐\n│a          │\n│few        │\n│lines      │\n│for testing│\n└───────────┘");
    }
}
