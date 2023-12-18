use crate::{oklaba::Oklaba, to_css_string::ToCssString, Hsla, Mix, SRgba};
use bevy::render::color::SrgbColorSpace;
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

/// Linear standard RGB color with alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(PartialEq, Serialize, Deserialize)]
pub struct LinearRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl LinearRgba {
    #[doc(alias = "transparent")]

    /// Construct a new LinearRgba color from components.
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Convert the [`LinearRgba`] color to a tuple of components (r, g, b, a). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Construct a new [`LinearRgba`] color from a tuple of components (r, g, b, a).
    #[inline]
    pub const fn from_components((red, green, blue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

impl Default for LinearRgba {
    fn default() -> Self {
        Self {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 1.,
        }
    }
}

impl ToCssString for LinearRgba {
    fn to_css_string(&self) -> String {
        format!(
            "color(srgb-linear {} {} {} {})",
            self.red * 255.0,
            self.green * 255.0,
            self.blue * 255.0,
            self.alpha
        )
    }
}

impl Mix for LinearRgba {
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

impl From<SRgba> for LinearRgba {
    #[inline]
    fn from(value: SRgba) -> Self {
        Self {
            red: value.red.nonlinear_to_linear_srgb(),
            green: value.green.nonlinear_to_linear_srgb(),
            blue: value.blue.nonlinear_to_linear_srgb(),
            alpha: value.alpha,
        }
    }
}

impl From<Oklaba> for LinearRgba {
    fn from(value: Oklaba) -> Self {
        let Oklaba { l, a, b, alpha } = value;

        // From https://github.com/Ogeon/palette
        let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

        let l = l_.powf(3.0);
        let m = m_.powf(3.0);
        let s = s_.powf(3.0);

        let red = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let green = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let blue = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl From<Hsla> for LinearRgba {
    #[inline]
    fn from(value: Hsla) -> Self {
        LinearRgba::from(SRgba::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_css_string() {
        assert_eq!(
            LinearRgba::from(SRgba::WHITE).to_css_string(),
            "color(srgb-linear 255 255 255 1)"
        );
        assert_eq!(
            LinearRgba::from(SRgba::RED).to_css_string(),
            "color(srgb-linear 255 0 0 1)"
        );
        assert_eq!(
            LinearRgba::from(SRgba::NONE).to_css_string(),
            "color(srgb-linear 0 0 0 0)"
        );
    }
}
