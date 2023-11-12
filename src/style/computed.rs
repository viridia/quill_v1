use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui::widget::UiImageSize;
use bevy::ui::ContentSize;

/// A computed style represents the composition of one or more `ElementStyle`s.
#[derive(Default, Clone, PartialEq, Debug)]
pub struct ComputedStyle {
    pub style: Style,

    // Text properties
    pub alignment: Option<TextAlignment>,
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font: Option<Handle<Font>>,
    pub line_break: Option<BreakLineOn>,

    // pub text_style: TextStyle,
    pub border_color: Option<Color>,
    pub background_color: Option<Color>,
    pub z_index: Option<i32>,

    // Image properties
    pub image: Option<Handle<Image>>,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl ComputedStyle {
    /// Construct a new, default style
    pub fn new() -> Self {
        Self { ..default() }
    }

    /// Construct a new style that inherits from a parent style. Only attributes which
    /// are inheritable will be inherited, all others will be set to the default.
    pub fn inherit(parent: &Self) -> Self {
        Self {
            alignment: parent.alignment,
            color: parent.color,
            font_size: parent.font_size,
            font: parent.font.clone(),
            line_break: parent.line_break.clone(),
            ..default()
        }
    }
}

/// Custom command that updates the style of an entity.
pub struct UpdateComputedStyle {
    pub(crate) entity: Entity,
    pub(crate) computed: ComputedStyle,
}

impl Command for UpdateComputedStyle {
    fn apply(self, world: &mut World) {
        if let Some(mut e) = world.get_entity_mut(self.entity) {
            if let Some(mut style) = e.get_mut::<Style>() {
                // Update the existing style
                if !style.eq(&self.computed.style) {
                    *style = self.computed.style;
                }
            } else {
                // Insert a new style component
                e.insert(self.computed.style);
            }

            match e.get_mut::<Text>() {
                Some(mut text) => {
                    // TODO: This is never executed
                    // TODO: Compare and mutate
                    if let Some(color) = self.computed.color {
                        for section in text.sections.iter_mut() {
                            if section.style.color != color {
                                section.style.color = color;
                            }
                        }
                    }

                    if let Some(ws) = self.computed.line_break {
                        if text.linebreak_behavior != ws {
                            text.linebreak_behavior = ws;
                        }
                    }
                }

                None => {}
            }

            match e.get_mut::<BackgroundColor>() {
                Some(mut bg_comp) => {
                    if self.computed.background_color.is_none() {
                        if self.computed.image.is_none() {
                            // Remove the background
                            e.remove::<BackgroundColor>();
                        }
                    } else {
                        let color = self.computed.background_color.unwrap();
                        // Mutate the background
                        if bg_comp.0 != color {
                            bg_comp.0 = color
                        }
                    }
                }

                None => {
                    if !self.computed.background_color.is_none() {
                        // Insert a new background
                        e.insert(BackgroundColor(self.computed.background_color.unwrap()));
                    } else if self.computed.image.is_some() {
                        // Images require a background color to be set.
                        e.insert(BackgroundColor::DEFAULT);
                    }
                }
            }

            match e.get_mut::<BorderColor>() {
                Some(mut bc_comp) => {
                    if self.computed.border_color.is_none() {
                        // Remove the border color
                        e.remove::<BorderColor>();
                    } else {
                        let color = self.computed.border_color.unwrap();
                        if bc_comp.0 != color {
                            bc_comp.0 = color
                        }
                    }
                }

                None => {
                    if !self.computed.border_color.is_none() {
                        // Insert a new background color
                        e.insert(BorderColor(self.computed.border_color.unwrap()));
                    }
                }
            }

            match e.get_mut::<UiImage>() {
                Some(mut img) => {
                    match self.computed.image {
                        Some(src) => {
                            if img.texture != src {
                                img.texture = src;
                            }
                            if img.flip_x != self.computed.flip_x {
                                img.flip_x = self.computed.flip_x;
                            }
                            if img.flip_y != self.computed.flip_y {
                                img.flip_y = self.computed.flip_y;
                            }
                        }
                        None => {
                            // Remove the image.
                            e.remove::<UiImage>();
                        }
                    }
                }

                None => {
                    if let Some(src) = self.computed.image {
                        // Create image component
                        e.insert((
                            UiImage {
                                texture: src,
                                flip_x: self.computed.flip_x,
                                flip_y: self.computed.flip_y,
                            },
                            ContentSize::default(),
                            UiImageSize::default(),
                        ));
                    }
                }
            }
        }
    }
}
