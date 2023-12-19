use crate::{to_css_string::*, LinearRgba, Mix, SRgba};
use bevy::render::color::HslRepresentation;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Color in Hue-Saturation-Lightness color space with alpha
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}

impl Hsla {
    /// Construct a new [`Hsla`] color from components.
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue channel. [0.0, 360.0]
    /// * `saturation` - Saturation channel. [0.0, 1.0]
    /// * `lightness` - Lightness channel. [0.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    /// Convert the Oklaba color to a tuple of components (h, s, l, a). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.hue, self.saturation, self.lightness, self.alpha)
    }

    /// Construct a new [`Oklaba`] color from a tuple of components (h, s, l, a).
    #[inline]
    pub const fn from_components((l, a, b, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(l, a, b, alpha)
    }
}

impl Default for Hsla {
    fn default() -> Self {
        Self::new(0., 0., 0., 1.)
    }
}

impl ToCssString for Hsla {
    fn to_css_string(&self) -> String {
        format!(
            "hsl({}deg {}% {}% {})",
            self.hue.round_to_decimal_places(6),
            (self.saturation * 100.).round_to_decimal_places(3),
            (self.lightness * 100.).round_to_decimal_places(3),
            self.alpha
        )
    }
}

impl Mix for Hsla {
    #[inline]
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        // TODO: Refactor this into EuclideanModulo::lerp_modulo
        let shortest_angle = ((((other.hue - self.hue) % 360.) + 540.) % 360.) - 180.;
        let mut hue = self.hue + shortest_angle * factor;
        if hue < 0. {
            hue += 360.;
        } else if hue >= 360. {
            hue -= 360.;
        }
        Self {
            hue,
            saturation: self.saturation * n_factor + other.saturation * factor,
            lightness: self.lightness * n_factor + other.lightness * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }
}

impl From<SRgba> for Hsla {
    fn from(value: SRgba) -> Self {
        let (h, s, l) =
            HslRepresentation::nonlinear_srgb_to_hsl([value.red, value.green, value.blue]);
        Self::new(h, s, l, value.alpha)
    }
}

impl From<LinearRgba> for Hsla {
    fn from(value: LinearRgba) -> Self {
        Hsla::from(SRgba::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testing::assert_approx_eq, SRgba};

    #[test]
    fn test_to_from_srgba() {
        let hsla = Hsla::new(0.5, 0.5, 0.5, 1.0);
        let srgba: SRgba = hsla.into();
        let hsla2: Hsla = srgba.into();
        assert_approx_eq!(hsla.hue, hsla2.hue, 0.001);
        assert_approx_eq!(hsla.saturation, hsla2.saturation, 0.001);
        assert_approx_eq!(hsla.lightness, hsla2.lightness, 0.001);
        assert_approx_eq!(hsla.alpha, hsla2.alpha, 0.001);
    }

    #[test]
    fn test_to_from_linear() {
        let hsla = Hsla::new(0.5, 0.5, 0.5, 1.0);
        let linear: LinearRgba = hsla.into();
        let hsla2: Hsla = linear.into();
        assert_approx_eq!(hsla.hue, hsla2.hue, 0.001);
        assert_approx_eq!(hsla.saturation, hsla2.saturation, 0.001);
        assert_approx_eq!(hsla.lightness, hsla2.lightness, 0.001);
        assert_approx_eq!(hsla.alpha, hsla2.alpha, 0.001);
    }

    #[test]
    fn to_css_string() {
        assert_eq!(
            Hsla::from(SRgba::WHITE).to_css_string(),
            "hsl(0deg 0% 100% 1)"
        );
        assert_eq!(
            Hsla::from(SRgba::RED).to_css_string(),
            "hsl(0deg 100% 50% 1)"
        );
        assert_eq!(
            Hsla::from(SRgba::BLUE).to_css_string(),
            "hsl(240deg 100% 50% 1)"
        );
        assert_eq!(Hsla::from(SRgba::NONE).to_css_string(), "hsl(0deg 0% 0% 0)");
    }

    #[test]
    fn test_mix_wrap() {
        let hsla0 = Hsla::new(10., 0.5, 0.5, 1.0);
        let hsla1 = Hsla::new(20., 0.5, 0.5, 1.0);
        let hsla2 = Hsla::new(350., 0.5, 0.5, 1.0);
        assert_approx_eq!(hsla0.mix(&hsla1, 0.25).hue, 12.5, 0.001);
        assert_approx_eq!(hsla0.mix(&hsla1, 0.5).hue, 15., 0.001);
        assert_approx_eq!(hsla0.mix(&hsla1, 0.75).hue, 17.5, 0.001);

        assert_approx_eq!(hsla1.mix(&hsla0, 0.25).hue, 17.5, 0.001);
        assert_approx_eq!(hsla1.mix(&hsla0, 0.5).hue, 15., 0.001);
        assert_approx_eq!(hsla1.mix(&hsla0, 0.75).hue, 12.5, 0.001);

        assert_approx_eq!(hsla0.mix(&hsla2, 0.25).hue, 5., 0.001);
        assert_approx_eq!(hsla0.mix(&hsla2, 0.5).hue, 0., 0.001);
        assert_approx_eq!(hsla0.mix(&hsla2, 0.75).hue, 355., 0.001);

        assert_approx_eq!(hsla2.mix(&hsla0, 0.25).hue, 355., 0.001);
        assert_approx_eq!(hsla2.mix(&hsla0, 0.5).hue, 0., 0.001);
        assert_approx_eq!(hsla2.mix(&hsla0, 0.75).hue, 5., 0.001);
    }
}
