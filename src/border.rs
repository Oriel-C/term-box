use super::AnsiStyle;

#[repr(usize)]
#[derive(Debug)]
pub(super) enum BorderChar {
    TopLeft = 0,
    TopRight = 1,
    Side = 2,
    BotLeft = 3,
    BotRight = 4,
    Edge = 5
}

/// Defines the shape of a [TermBox's](super::TermBox) border.
#[derive(Debug, Clone, Copy, Default)]
pub enum BorderShape {
    /// Use a single line for the border: \
    /// ┌─┐ \
    /// └─┘
    #[default]
    Single,
    /// Use a double line for the border: \
    /// ╔═╗ \
    /// ╚═╝
    Double
}

impl BorderShape {
    const SINGLE_SHAPES: [&str; 6] = [ "┌", "┐", "│", "└", "┘", "─" ];
    const DOUBLE_SHAPES: [&str; 6] = [ "╔", "╗", "║", "╚", "╝", "═" ];

    pub(super) fn get_char(self, char: BorderChar) -> &'static str {
        match self {
            Self::Single => Self::SINGLE_SHAPES[char as usize],
            Self::Double => Self::DOUBLE_SHAPES[char as usize]
        }
    }
}

/// Style for a [TermBox's](super::TermBox) border, determing the [shape](BorderStyle::shape)
/// and [style](BorderStyle::style) of the border.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderStyle {
    pub(super) shape: BorderShape,
    pub(super) style: AnsiStyle
}

impl BorderStyle {
    /// Creates a new [BorderStyle] with [BorderShape::Single]
    /// and no ANSI styling.
    pub fn new_single() -> Self {
        Self::default()
    }

    /// Creates a new [BorderStyle] with [BorderShape::Double]
    /// and no ANSI styling.
    pub fn new_double() -> Self {
        Self { shape: BorderShape::Double, style: AnsiStyle::default() }
    }

    /// Sets the [AnsiStyle] for the border and returns it.
    pub fn with_style(mut self, style: impl Into<AnsiStyle>) -> Self {
        self.style = style.into();
        self
    }

    pub(super) fn get_edge_string(&self) -> String {
        let base = self.shape.get_char(BorderChar::Side);
        self.style.paint(base).to_string()
    }

    /// Returns the [BorderShape] for the border.
    pub fn shape(&self) -> BorderShape {
        self.shape
    }

    /// Returns the [AnsiStyle] for the border.
    pub fn style(&self) -> AnsiStyle {
        self.style
    }
}
