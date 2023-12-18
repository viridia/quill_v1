use crate::{oklaba::Oklaba, Hsla, LinearRgba, Mix, SRgba};

/// Represents a range of colors that can be linearly interpolated, defined by a start and
/// end point which must be in the same color space.
pub struct ColorRange<T: Mix> {
    start: T,
    end: T,
}

impl<T> ColorRange<T>
where
    T: Mix,
{
    /// Construct a new color range from the start and end values.
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    /// Get the color value at the given interpolation factor, which should be between 0.0
    /// and 1.0.
    pub fn at(&self, factor: f32) -> T {
        self.start.mix(&self.end, factor)
    }
}

/// A type-erased color range that can be used to interpolate between colors in different
/// color spaces.
pub trait AnyColorRange {
    fn at_linear(&self, factor: f32) -> LinearRgba;
}

impl AnyColorRange for ColorRange<LinearRgba> {
    fn at_linear(&self, factor: f32) -> LinearRgba {
        self.at(factor)
    }
}

impl AnyColorRange for ColorRange<SRgba> {
    fn at_linear(&self, factor: f32) -> LinearRgba {
        self.at(factor).into()
    }
}

impl AnyColorRange for ColorRange<Oklaba> {
    fn at_linear(&self, factor: f32) -> LinearRgba {
        self.at(factor).into()
    }
}

impl AnyColorRange for ColorRange<Hsla> {
    fn at_linear(&self, factor: f32) -> LinearRgba {
        self.at(factor).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LinearRgba, SRgba};

    #[test]
    fn test_color_range() {
        let range = ColorRange::new(SRgba::RED, SRgba::BLUE);
        assert_eq!(range.at(0.0), SRgba::RED);
        assert_eq!(range.at(0.5), SRgba::new(0.5, 0.0, 0.5, 1.0));
        assert_eq!(range.at(1.0), SRgba::BLUE);

        let lred: LinearRgba = SRgba::RED.into();
        let lblue: LinearRgba = SRgba::BLUE.into();

        let range = ColorRange::new(lred, lblue);
        assert_eq!(range.at(0.0), lred);
        assert_eq!(range.at(0.5), LinearRgba::new(0.5, 0.0, 0.5, 1.0));
        assert_eq!(range.at(1.0), lblue);
    }
}
