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

impl BorderChar {
    pub const LEN_BYTES: usize = BorderShape::SINGLE_SHAPES[0].len();
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

    pub(super) fn get_char(self, char: BorderChar) -> &'static str {
        match self {
            Self::Single => Self::SINGLE_SHAPES[char as usize],
            Self::Double => Self::DOUBLE_SHAPES[char as usize]
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BorderStyle {
    pub(super) shape: BorderShape,
    pub(super) style: AnsiStyle
}

impl BorderStyle {
    pub fn new_single() -> Self {
        Self::default()
    }

    pub fn new_double() -> Self {
        Self { shape: BorderShape::Double, style: AnsiStyle::default() }
    }

    pub fn with_style(mut self, style: impl Into<AnsiStyle>) -> Self {
        self.style = style.into();
        self
    }

    pub(super) fn get_edge_string(&self) -> String {
        let base = self.shape.get_char(BorderChar::Side);
        self.style.paint(base).to_string()
    }

    pub fn shape(&self) -> BorderShape {
        self.shape
    }

    pub fn style(&self) -> AnsiStyle {
        self.style
    }
}


