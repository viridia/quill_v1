/// Which side of the anchor the floating element should be placed.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FloatSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// How the floating element should be aligned to the anchor.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FloatAlign {
    #[default]
    Start,
    End,
    Center,
}

pub struct FloatPosition {
    /// The side of the anchor the floating element should be placed.
    pub side: FloatSide,

    /// How the floating element should be aligned to the anchor.
    pub align: FloatAlign,

    /// If true, the floating element will be at least as large as the anchor on the adjacent
    /// side.
    pub stretch: bool,

    /// The gap between the anchor and the floating element.
    pub gap: f32,
}
