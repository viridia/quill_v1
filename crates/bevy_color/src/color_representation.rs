use crate::{to_css_string::ToCssString, Hsla, LinearRgba, Oklaba, SRgba};

/// An enumerated type that can represent any of the color types in this crate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorRepresentation {
    SRgba(SRgba),
    LinearRgba(LinearRgba),
    Hsla(Hsla),
    Oklaba(Oklaba),
}

impl ColorRepresentation {
    /// Return the color as a linear RGBA color.
    pub fn linear(&self) -> LinearRgba {
        match self {
            ColorRepresentation::SRgba(srgba) => (*srgba).into(),
            ColorRepresentation::LinearRgba(linear) => (*linear).into(),
            ColorRepresentation::Hsla(hsla) => (*hsla).into(),
            ColorRepresentation::Oklaba(oklab) => (*oklab).into(),
        }
    }
}

impl Default for ColorRepresentation {
    fn default() -> Self {
        Self::SRgba(SRgba::WHITE)
    }
}

impl ToCssString for ColorRepresentation {
    fn to_css_string(&self) -> String {
        match self {
            ColorRepresentation::SRgba(srgba) => srgba.to_css_string(),
            ColorRepresentation::LinearRgba(linear) => linear.to_css_string(),
            ColorRepresentation::Hsla(hsla) => hsla.to_css_string(),
            ColorRepresentation::Oklaba(oklab) => oklab.to_css_string(),
        }
    }
}

impl From<SRgba> for ColorRepresentation {
    fn from(value: SRgba) -> Self {
        Self::SRgba(value)
    }
}

impl From<LinearRgba> for ColorRepresentation {
    fn from(value: LinearRgba) -> Self {
        Self::LinearRgba(value)
    }
}

impl From<Hsla> for ColorRepresentation {
    fn from(value: Hsla) -> Self {
        Self::Hsla(value)
    }
}

impl From<Oklaba> for ColorRepresentation {
    fn from(value: Oklaba) -> Self {
        Self::Oklaba(value)
    }
}
