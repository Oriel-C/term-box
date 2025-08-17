pub use nu_ansi_term::{Color, Style as AnsiStyle};
use ansi_width::ansi_width;

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

#[derive(Debug, Clone, Default)]
pub struct Box {
    border_style: BorderStyle,
    lines: Vec<String>
}

impl Box {
    pub fn print(&self) {
        print!("{}", self.to_string())
    }

    pub fn to_string(&self) -> String {
        let longest_line = self.lines
            .iter()
            .map(|a| ansi_width(a))
            .max()
            .unwrap_or_default();
        let line_len = longest_line + 3; // + 3: |,|,\n

        // TODO estimate capacity needed for ANSI control sequences
        let mut buf = String::with_capacity((self.lines.len() + 2) * line_len);

        make_top_line(&mut buf, self.border_style, line_len);
        make_bottom_line(&mut buf, self.border_style, line_len);

        buf
    }
}

fn make_top_line(buf: &mut String, style: BorderStyle, len: usize) {
    make_top_or_bottom_line(buf, style, len, BorderChar::TopLeft, BorderChar::TopRight)
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
    buf.push('\n')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let box_ = Box::default().to_string();
        assert_eq!(box_, "┌─┐\n└─┘\n", "{}", box_.chars().count());
    }
}
