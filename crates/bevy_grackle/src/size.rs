// TODO: This belongs in Grackle, not Egret

/// Standard sizes for buttons and other widgets that have size variants.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Size {
    Xl,
    Lg,
    #[default]
    Md,
    Sm,
    Xs,
    Xxs,
    Xxxs,
}

impl Size {
    /// Class name for Size.
    pub fn class_name(&self) -> &'static str {
        match self {
            Size::Xl => "size-xl",
            Size::Lg => "size-lg",
            Size::Md => "size-md",
            Size::Sm => "size-sm",
            Size::Xs => "size-xs",
            Size::Xxs => "size-xxs",
            Size::Xxxs => "size-xxxs",
        }
    }

    /// Returns the height of the widget in pixels.
    pub fn height(&self) -> f32 {
        match self {
            Size::Xl => 2.5 * 16.0,
            Size::Lg => 2.2 * 16.0,
            Size::Md => 2.0 * 16.0,
            Size::Sm => 1.85 * 16.0,
            Size::Xs => 1.65 * 16.0,
            Size::Xxs => 1.45 * 16.0,
            Size::Xxxs => 1.3 * 16.0,
        }
    }

    /// Returns the desired font size for the widget.
    pub fn font_size(&self) -> f32 {
        match self {
            Size::Xl => 18.0,
            Size::Lg => 16.0,
            Size::Md => 15.0,
            Size::Sm => 14.0,
            Size::Xs => 13.0,
            Size::Xxs => 12.0,
            Size::Xxxs => 11.0,
        }
    }
}

// export type Space = 'xl' | 'lg' | 'md' | 'sm' | 'xs' | 'none';
// export type DialogWidth = Size | 'full';
