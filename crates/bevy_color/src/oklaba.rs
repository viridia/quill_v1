use crate::{
    to_css_string::{RoundToDecimalPlaces, ToCssString},
    LinearRgba, Mix, SRgba,
};
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Color in Oklaba color space, with alpha
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct Oklaba {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

impl Oklaba {
    /// Construct a new [`Oklaba`] color from components.
    ///
    /// # Arguments
    ///
    /// * `l` - Lightness channel. [0.0, 1.0]
    /// * `a` - Green-red channel. [-1.0, 1.0]
    /// * `b` - Blue-yellow channel. [-1.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self { l, a, b, alpha }
    }

    /// Convert the Oklaba color to a tuple of components (l, a, b, alpha). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.l, self.a, self.b, self.alpha)
    }

    /// Construct a new [`Oklaba`] color from a tuple of components (l, a, b, alpha).
    #[inline]
    pub const fn from_components((l, a, b, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(l, a, b, alpha)
    }
}

impl Default for Oklaba {
    fn default() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}

impl ToCssString for Oklaba {
    fn to_css_string(&self) -> String {
        format!(
            "color(oklab {}% {} {} {})",
            (self.l * 100.0).round_to_decimal_places(3),
            self.a.round_to_decimal_places(6),
            self.b.round_to_decimal_places(6),
            self.alpha
        )
    }
}

impl Mix for Oklaba {
    #[inline]
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        Self {
            l: self.l * n_factor + other.l * factor,
            a: self.a * n_factor + other.a * factor,
            b: self.b * n_factor + other.b * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }
}

impl From<LinearRgba> for Oklaba {
    fn from(value: LinearRgba) -> Self {
        let LinearRgba {
            red,
            green,
            blue,
            alpha,
        } = value;
        // From https://github.com/DougLau/pix
        let l = 0.4122214708 * red + 0.5363325363 * green + 0.0514459929 * blue;
        let m = 0.2119034982 * red + 0.6806995451 * green + 0.1073969566 * blue;
        let s = 0.0883024619 * red + 0.2817188376 * green + 0.6299787005 * blue;
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();
        let l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
        Oklaba::new(l, a, b, alpha)
    }
}

impl From<SRgba> for Oklaba {
    fn from(value: SRgba) -> Self {
        Oklaba::from(LinearRgba::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testing::assert_approx_eq, SRgba};

    #[test]
    fn test_to_from_srgba() {
        let oklab: Oklaba = SRgba::RED.into();
        assert_eq!(oklab, Oklaba::new(0.6279554, 0.22486295, 0.1258463, 1.0));

        let oklaba = Oklaba::new(0.5, 0.5, 0.5, 1.0);
        let srgba: SRgba = oklaba.into();
        let oklaba2: Oklaba = srgba.into();
        assert_approx_eq!(oklaba.l, oklaba2.l, 0.001);
        assert_approx_eq!(oklaba.a, oklaba2.a, 0.001);
        assert_approx_eq!(oklaba.b, oklaba2.b, 0.001);
        assert_approx_eq!(oklaba.alpha, oklaba2.alpha, 0.001);
    }

    #[test]
    fn test_to_from_linear() {
        let oklaba = Oklaba::new(0.5, 0.5, 0.5, 1.0);
        let linear: LinearRgba = oklaba.into();
        let oklaba2: Oklaba = linear.into();
        assert_approx_eq!(oklaba.l, oklaba2.l, 0.001);
        assert_approx_eq!(oklaba.a, oklaba2.a, 0.001);
        assert_approx_eq!(oklaba.b, oklaba2.b, 0.001);
        assert_approx_eq!(oklaba.alpha, oklaba2.alpha, 0.001);
    }

    #[test]
    fn to_css_string() {
        assert_eq!(
            Oklaba::from(SRgba::WHITE).to_css_string(),
            "color(oklab 100% 0 0 1)"
        );
        assert_eq!(
            Oklaba::from(SRgba::RED).to_css_string(),
            "color(oklab 62.796% 0.224863 0.125846 1)"
        );
        assert_eq!(
            Oklaba::from(SRgba::NONE).to_css_string(),
            "color(oklab 0% 0 0 0)"
        );
    }
}
