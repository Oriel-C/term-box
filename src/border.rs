/*! Defines the [BorderStyle] and related [BorderShape] type for describing
 * the look of a box's border. */

use super::AnsiStyle;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub(super) enum BorderChar {
    TopLeft = 0,
    TopRight = 1,
    Side = 2,
    BotLeft = 3,
    BotRight = 4,
    Edge = 5
}

impl BorderChar {
    pub const NUM_BYTES: usize = BorderShape::SINGLE_SHAPES[0].len();
}

/// Defines the shape of a [TermBox's](super::TermBox) border.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BorderShape {
    /** Use a single line for the border: \
    * ┌─┐ \
    * └─┘ \
    * Gaps displayed in documentation will not appear in most terminals, font-dependent.
    */
    #[default]
    Single,

    /** Use a double line for the border: \
    * ╔═╗ \
    * ╚═╝ \
    * Gaps displayed in documentation will not appear in most terminals, font-dependent.
    */
    Double
}

type Shapes = [&'static str; 6];

impl BorderShape {
    const SINGLE_SHAPES: Shapes = [ "┌", "┐", "│", "└", "┘", "─" ];
    const DOUBLE_SHAPES: Shapes = [ "╔", "╗", "║", "╚", "╝", "═" ];

    pub(super) fn get_char(self, char: BorderChar) -> &'static str {
        match self {
            Self::Single => Self::SINGLE_SHAPES[char as usize],
            Self::Double => Self::DOUBLE_SHAPES[char as usize]
        }
    }
}

/// Style for a [TermBox's](super::TermBox) border, determing the [shape](BorderStyle::shape)
/// and [style](BorderStyle::ansi_style) of the border.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BorderStyle {
    pub(super) shape: BorderShape,
    pub(super) ansi: AnsiStyle
}

impl BorderStyle {
    /// Creates a new [BorderStyle] with [BorderShape::Single] and no ANSI styling.
    pub fn new_single() -> Self { Self::default() }

    /// Creates a new [BorderStyle] with [BorderShape::Double] and no ANSI styling.
    pub fn new_double() -> Self {
        Self { shape: BorderShape::Double, ansi: AnsiStyle::default() }
    }

    /** Sets the [AnsiStyle] for the border and returns it. \
    * Styling may not appear properly outside of a terminal.
    */
    pub fn with_style(mut self, style: impl Into<AnsiStyle>) -> Self {
        self.ansi = style.into();
        self
    }

    pub(super) fn get_edge_string(&self) -> String {
        let base = self.shape.get_char(BorderChar::Side);
        self.ansi.paint(base).to_string()
    }

    /// Returns the [BorderShape] for the border.
    pub fn shape(&self) -> BorderShape { self.shape }

    /// Returns the [AnsiStyle] for the border.
    pub fn ansi_style(&self) -> AnsiStyle { self.ansi }
}
