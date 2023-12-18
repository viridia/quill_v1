use crate::oklaba::Oklaba;
use crate::to_css_string::ToCssString;
use crate::{Hsla, LinearRgba, Mix};
use bevy::render::color::{HexColorError, HslRepresentation, SrgbColorSpace};
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
    /// <div style="background-color:rgb(94%, 97%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ALICE_BLUE: SRgba = SRgba::new(0.94, 0.97, 1.0, 1.0);
    /// <div style="background-color:rgb(98%, 92%, 84%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ANTIQUE_WHITE: SRgba = SRgba::new(0.98, 0.92, 0.84, 1.0);
    /// <div style="background-color:rgb(49%, 100%, 83%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const AQUAMARINE: SRgba = SRgba::new(0.49, 1.0, 0.83, 1.0);
    /// <div style="background-color:rgb(94%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const AZURE: SRgba = SRgba::new(0.94, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(96%, 96%, 86%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BEIGE: SRgba = SRgba::new(0.96, 0.96, 0.86, 1.0);
    /// <div style="background-color:rgb(100%, 89%, 77%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BISQUE: SRgba = SRgba::new(1.0, 0.89, 0.77, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLACK: SRgba = SRgba::new(0.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLUE: SRgba = SRgba::new(0.0, 0.0, 1.0, 1.0);
    /// <div style="background-color:rgb(86%, 8%, 24%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const CRIMSON: SRgba = SRgba::new(0.86, 0.08, 0.24, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const CYAN: SRgba = SRgba::new(0.0, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(25%, 25%, 25%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const DARK_GRAY: SRgba = SRgba::new(0.25, 0.25, 0.25, 1.0);
    /// <div style="background-color:rgb(0%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const DARK_GREEN: SRgba = SRgba::new(0.0, 0.5, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const FUCHSIA: SRgba = SRgba::new(1.0, 0.0, 1.0, 1.0);
    /// <div style="background-color:rgb(100%, 84%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GOLD: SRgba = SRgba::new(1.0, 0.84, 0.0, 1.0);
    /// <div style="background-color:rgb(50%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GRAY: SRgba = SRgba::new(0.5, 0.5, 0.5, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GREEN: SRgba = SRgba::new(0.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(28%, 0%, 51%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const INDIGO: SRgba = SRgba::new(0.29, 0.0, 0.51, 1.0);
    /// <div style="background-color:rgb(20%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const LIME_GREEN: SRgba = SRgba::new(0.2, 0.8, 0.2, 1.0);
    /// <div style="background-color:rgb(50%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const MAROON: SRgba = SRgba::new(0.5, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(10%, 10%, 44%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const MIDNIGHT_BLUE: SRgba = SRgba::new(0.1, 0.1, 0.44, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const NAVY: SRgba = SRgba::new(0.0, 0.0, 0.5, 1.0);
    /// <div style="background-color:rgba(0%, 0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    #[doc(alias = "transparent")]
    pub const NONE: SRgba = SRgba::new(0.0, 0.0, 0.0, 0.0);
    /// <div style="background-color:rgb(50%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const OLIVE: SRgba = SRgba::new(0.5, 0.5, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 65%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ORANGE: SRgba = SRgba::new(1.0, 0.65, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 27%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const ORANGE_RED: SRgba = SRgba::new(1.0, 0.27, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 8%, 57%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PINK: SRgba = SRgba::new(1.0, 0.08, 0.58, 1.0);
    /// <div style="background-color:rgb(50%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PURPLE: SRgba = SRgba::new(0.5, 0.0, 0.5, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RED: SRgba = SRgba::new(1.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(98%, 50%, 45%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SALMON: SRgba = SRgba::new(0.98, 0.5, 0.45, 1.0);
    /// <div style="background-color:rgb(18%, 55%, 34%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SEA_GREEN: SRgba = SRgba::new(0.18, 0.55, 0.34, 1.0);
    /// <div style="background-color:rgb(75%, 75%, 75%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SILVER: SRgba = SRgba::new(0.75, 0.75, 0.75, 1.0);
    /// <div style="background-color:rgb(0%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TEAL: SRgba = SRgba::new(0.0, 0.5, 0.5, 1.0);
    /// <div style="background-color:rgb(100%, 39%, 28%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TOMATO: SRgba = SRgba::new(1.0, 0.39, 0.28, 1.0);
    /// <div style="background-color:rgb(25%, 88%, 82%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TURQUOISE: SRgba = SRgba::new(0.25, 0.88, 0.82, 1.0);
    /// <div style="background-color:rgb(93%, 51%, 93%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const VIOLET: SRgba = SRgba::new(0.93, 0.51, 0.93, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const WHITE: SRgba = SRgba::new(1.0, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YELLOW: SRgba = SRgba::new(1.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(60%, 80%, 20%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YELLOW_GREEN: SRgba = SRgba::new(0.6, 0.8, 0.2, 1.0);

    /// Construct a new [`SRgba`] color from components.
    ///
    /// # Arguments
    ///
    /// * `red` - Red channel. [0.0, 1.0]
    /// * `green` - Green channel. [0.0, 1.0]
    /// * `blue` - Blue channel. [0.0, 1.0]
    /// * `alpha` - Alpha channel. [0.0, 1.0]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Convert the [`SRgba`] color to a tuple of components (r, g, b, a). This is useful
    /// when you need to transmute the data type of a color to a different type without converting
    /// the values.
    /// For example, you can convert an [`SRgba`] to [`LinearRgba`] by doing:
    ///
    /// ```
    /// # use bevy_color::SRgba;
    /// # use bevy_color::LinearRgba;
    /// let srgba = SRgba::new(0.0, 0.5, 1.0, 1.0);
    /// let linear_rgba: LinearRgba = LinearRgba::from_components(srgba.to_components());
    /// ```
    #[inline]
    pub const fn to_components(&self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alpha)
    }

    /// Construct a new [`SRgba`] color from a tuple of components (r, g, b, a). This is
    /// the converse of `from_components`.
    #[inline]
    pub const fn from_components((red, green, blue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(red, green, blue, alpha)
    }

    /// New `SRgba` from sRGB colorspace.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_color::SRgba;
    /// let color = SRgba::hex("FF00FF").unwrap(); // fuchsia
    /// let color = SRgba::hex("FF00FF7F").unwrap(); // partially transparent fuchsia
    ///
    /// // A standard hex color notation is also available
    /// assert_eq!(SRgba::hex("#FFFFFF").unwrap(), SRgba::new(1.0, 1.0, 1.0, 1.0));
    /// ```
    ///
    pub fn hex<T: AsRef<str>>(hex: T) -> Result<Self, HexColorError> {
        let hex = hex.as_ref();
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match *hex.as_bytes() {
            // RGB
            [r, g, b] => {
                let [r, g, b, ..] = decode_hex([r, r, g, g, b, b])?;
                Ok(Self::rgb_u8(r, g, b))
            }
            // RGBA
            [r, g, b, a] => {
                let [r, g, b, a, ..] = decode_hex([r, r, g, g, b, b, a, a])?;
                Ok(Self::rgba_u8(r, g, b, a))
            }
            // RRGGBB
            [r1, r2, g1, g2, b1, b2] => {
                let [r, g, b, ..] = decode_hex([r1, r2, g1, g2, b1, b2])?;
                Ok(Self::rgb_u8(r, g, b))
            }
            // RRGGBBAA
            [r1, r2, g1, g2, b1, b2, a1, a2] => {
                let [r, g, b, a, ..] = decode_hex([r1, r2, g1, g2, b1, b2, a1, a2])?;
                Ok(Self::rgba_u8(r, g, b, a))
            }
            _ => Err(HexColorError::Length),
        }
    }

    /// New `SRgba` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    ///
    /// See also [`SRgba::rgb`], [`SRgba::rgba_u8`], [`SRgba::hex`].
    ///
    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::rgba_u8(r, g, b, u8::MAX)
    }

    // Float operations in const fn are not stable yet
    // see https://github.com/rust-lang/rust/issues/57241
    /// New `SRgba` from sRGB colorspace.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel. [0, 255]
    /// * `g` - Green channel. [0, 255]
    /// * `b` - Blue channel. [0, 255]
    /// * `a` - Alpha channel. [0, 255]
    ///
    /// See also [`SRgba::rgba`], [`SRgba::rgb_u8`], [`SRgba::hex`].
    ///
    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / u8::MAX as f32,
            g as f32 / u8::MAX as f32,
            b as f32 / u8::MAX as f32,
            a as f32 / u8::MAX as f32,
        )
    }
}

impl Default for SRgba {
    fn default() -> Self {
        Self::WHITE
    }
}

impl ToCssString for SRgba {
    fn to_css_string(&self) -> String {
        format!(
            "rgba({} {} {} {})",
            self.red * 255.0,
            self.green * 255.0,
            self.blue * 255.0,
            self.alpha
        )
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

/// Converts hex bytes to an array of RGB\[A\] components
///
/// # Example
/// For RGB: *b"ffffff" -> [255, 255, 255, ..]
/// For RGBA: *b"E2E2E2FF" -> [226, 226, 226, 255, ..]
const fn decode_hex<const N: usize>(mut bytes: [u8; N]) -> Result<[u8; N], HexColorError> {
    let mut i = 0;
    while i < bytes.len() {
        // Convert single hex digit to u8
        let val = match hex_value(bytes[i]) {
            Ok(val) => val,
            Err(byte) => return Err(HexColorError::Char(byte as char)),
        };
        bytes[i] = val;
        i += 1;
    }
    // Modify the original bytes to give an `N / 2` length result
    i = 0;
    while i < bytes.len() / 2 {
        // Convert pairs of u8 to R/G/B/A
        // e.g `ff` -> [102, 102] -> [15, 15] = 255
        bytes[i] = bytes[i * 2] * 16 + bytes[i * 2 + 1];
        i += 1;
    }
    Ok(bytes)
}

/// Parse a single hex digit (a-f/A-F/0-9) as a `u8`
const fn hex_value(b: u8) -> Result<u8, u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        // Wrong hex digit
        _ => Err(b),
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

    #[test]
    fn hex_color() {
        assert_eq!(SRgba::hex("FFF"), Ok(SRgba::WHITE));
        assert_eq!(SRgba::hex("FFFF"), Ok(SRgba::WHITE));
        assert_eq!(SRgba::hex("FFFFFF"), Ok(SRgba::WHITE));
        assert_eq!(SRgba::hex("FFFFFFFF"), Ok(SRgba::WHITE));
        assert_eq!(SRgba::hex("000"), Ok(SRgba::BLACK));
        assert_eq!(SRgba::hex("000F"), Ok(SRgba::BLACK));
        assert_eq!(SRgba::hex("000000"), Ok(SRgba::BLACK));
        assert_eq!(SRgba::hex("000000FF"), Ok(SRgba::BLACK));
        assert_eq!(SRgba::hex("03a9f4"), Ok(SRgba::rgb_u8(3, 169, 244)));
        assert_eq!(SRgba::hex("yy"), Err(HexColorError::Length));
        assert_eq!(SRgba::hex("yyy"), Err(HexColorError::Char('y')));
        assert_eq!(SRgba::hex("#f2a"), Ok(SRgba::rgb_u8(255, 34, 170)));
        assert_eq!(SRgba::hex("#e23030"), Ok(SRgba::rgb_u8(226, 48, 48)));
        assert_eq!(SRgba::hex("#ff"), Err(HexColorError::Length));
        assert_eq!(SRgba::hex("##fff"), Err(HexColorError::Char('#')));
    }

    #[test]
    fn to_css_string() {
        assert_eq!(SRgba::WHITE.to_css_string(), "rgba(255 255 255 1)");
        assert_eq!(SRgba::RED.to_css_string(), "rgba(255 0 0 1)");
        assert_eq!(SRgba::NONE.to_css_string(), "rgba(0 0 0 0)");
    }
}
