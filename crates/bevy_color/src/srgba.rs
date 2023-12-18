use crate::colorspace::SrgbColorSpace;
use crate::oklaba::Oklaba;
use crate::{Hsla, LinearRgba, Mix};
use bevy::render::color::HslRepresentation;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Non-linear standard RGB with alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct SRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl SRgba {
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLUE: SRgba = SRgba::new(0.0, 0.0, 1.0, 1.0);
    #[doc(alias = "transparent")]
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const NONE: SRgba = SRgba::new(0.0, 0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RED: SRgba = SRgba::new(1.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const WHITE: SRgba = SRgba::new(1.0, 1.0, 1.0, 1.0);

    /// Construct a new [`SRgba`] color from components.
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Convert the [`SRgba`] color to a tuple of components.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Construct a new [`SRgba`] color from components.
    #[inline]
    pub const fn from_components((red, green, blue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

impl Default for SRgba {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Mix for SRgba {
    #[inline]
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let n_factor = 1.0 - factor;
        Self {
            red: self.red * n_factor + other.red * factor,
            green: self.green * n_factor + other.green * factor,
            blue: self.blue * n_factor + other.blue * factor,
            alpha: self.alpha * n_factor + other.alpha * factor,
        }
    }
}

impl From<LinearRgba> for SRgba {
    #[inline]
    fn from(value: LinearRgba) -> Self {
        Self {
            red: value.red.linear_to_nonlinear_srgb(),
            green: value.green.linear_to_nonlinear_srgb(),
            blue: value.blue.linear_to_nonlinear_srgb(),
            alpha: value.alpha,
        }
    }
}

impl From<Hsla> for SRgba {
    fn from(value: Hsla) -> Self {
        let [r, g, b] =
            HslRepresentation::hsl_to_nonlinear_srgb(value.hue, value.saturation, value.lightness);
        Self::new(r, g, b, value.alpha)
    }
}

impl From<Oklaba> for SRgba {
    fn from(value: Oklaba) -> Self {
        SRgba::from(LinearRgba::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn test_to_from_linear() {
        let srgba = SRgba::new(0.0, 0.5, 1.0, 1.0);
        let linear_rgba: LinearRgba = srgba.into();
        assert_eq!(linear_rgba.red, 0.0);
        assert_approx_eq!(linear_rgba.green, 0.2140, 0.0001);
        assert_approx_eq!(linear_rgba.blue, 1.0, 0.0001);
        assert_eq!(linear_rgba.alpha, 1.0);
        let srgba2: SRgba = linear_rgba.into();
        assert_eq!(srgba2.red, 0.0);
        assert_approx_eq!(srgba2.green, 0.5, 0.0001);
        assert_approx_eq!(srgba2.blue, 1.0, 0.0001);
        assert_eq!(srgba2.alpha, 1.0);
    }
}
