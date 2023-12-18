use crate::{LinearRgba, Mix, SRgba};
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
    pub const fn new(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        Self { l, a, b, alpha }
    }

    /// Convert the Oklaba color to a tuple of components.
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.l, self.a, self.b, self.alpha)
    }

    /// Construct a new [`Oklaba`] color from components.
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
        let l = l.cbrt();
        let m = m.cbrt();
        let s = s.cbrt();
        let l = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s;
        let a = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s;
        let b = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s;
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
    use crate::SRgba;

    macro_rules! assert_approx_eq {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn test_to_from_srgba() {
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
}
