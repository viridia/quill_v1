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

/// Helper trait for rounding a float to three decimal places.
pub(crate) trait RoundToThousandths {
    fn round_to_thousandths(&self) -> f32;
}

impl RoundToThousandths for f32 {
    fn round_to_thousandths(&self) -> f32 {
        (self * 1000.0).round() / 1000.0
    }
}
