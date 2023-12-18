/// Trait for converting a type to a CSS color string.
pub trait ToCssString {
    /// Returns the CSS string representation of the color, for example 'rgba(255, 255, 255, 1.0)'.
    ///
    /// Examples:
    /// ```
    /// use bevy_color::SRgba;
    /// use bevy_color::Oklaba;
    /// use bevy_color::ToCssString;
    /// let css = SRgba::WHITE.to_css_string(); // "rgba(255 0 0 1)"
    /// let css = Oklaba::from(SRgba::RED).to_css_string(); // "color(oklab 62.796% -0.005 0.123 1)"
    /// ```
    fn to_css_string(&self) -> String;
}

/// Helper trait for rounding a float to a specified number of decimal places. Used in cases where
/// the color component is shown as a percentage, and we want to avoid excess precision when
/// encoding.
pub(crate) trait RoundToDecimalPlaces {
    fn round_to_decimal_places(&self, decimals: u32) -> f32;
}

impl RoundToDecimalPlaces for f32 {
    fn round_to_decimal_places(&self, decimals: u32) -> f32 {
        let factor = 10.0_f32.powi(decimals as i32);
        (self * factor).round() / factor
    }
}
