use bevy::asset::AssetPath;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use bevy::text::BreakLineOn;
use bevy::ui::widget::UiImageSize;
use bevy::ui::ContentSize;
// use bevy::ui::widget::UiImageSize;
// use bevy::ui::ContentSize;
use bevy_mod_picking::prelude::Pickable;

use super::style::PointerEvents;
use super::transition::{
    AnimatedBackgroundColor, AnimatedBorderColor, AnimatedTransform, Transition,
    TransitionProperty, TransitionState,
};

/// A computed style represents the composition of one or more `ElementStyle`s.
#[derive(Default, Clone, Debug)]
#[doc(hidden)]
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
    pub outline_color: Option<Color>,
    pub outline_width: Val,
    pub outline_offset: Val,
    pub z_index: Option<ZIndex>,

    // Transform properties
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub rotation: Option<f32>,
    pub translation: Option<Vec3>,

    // Image properties
    pub image: Option<AssetPath<'static>>,
    pub flip_x: bool,
    pub flip_y: bool,

    // Picking properties
    pub pickable: Option<PointerEvents>,

    // Transitiions
    pub transitions: Vec<Transition>,
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
        let mut is_animated_bg_color = false;
        let mut is_animated_border_color = false;
        let mut is_animated_transform = false;

        self.computed
            .transitions
            .iter()
            .for_each(|tr| match tr.property {
                TransitionProperty::Transform => is_animated_transform = true,
                TransitionProperty::BackgroundColor => is_animated_bg_color = true,
                TransitionProperty::BorderColor => is_animated_border_color = true,
            });

        let bg_image = self.computed.image.map(|path| {
            world
                .get_resource::<AssetServer>()
                .unwrap()
                .load_with_settings(path, |s: &mut ImageLoaderSettings| {
                    s.sampler = ImageSampler::linear()
                })
        });
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

            if is_animated_bg_color {
                match e.get_mut::<AnimatedBackgroundColor>() {
                    Some(_) => todo!(),
                    None => todo!(),
                }
            } else {
                e.remove::<AnimatedBackgroundColor>();
                match e.get_mut::<BackgroundColor>() {
                    Some(mut bg_comp) => {
                        if self.computed.background_color.is_none() {
                            if bg_image.is_none() {
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
                        } else if bg_image.is_some() {
                            // Images require a background color to be set.
                            e.insert(BackgroundColor::DEFAULT);
                        }
                    }
                }
            }

            if is_animated_border_color {
                match e.get_mut::<AnimatedBorderColor>() {
                    Some(_) => todo!(),
                    None => todo!(),
                }
            } else {
                e.remove::<AnimatedBorderColor>();
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
            }

            match e.get_mut::<UiImage>() {
                Some(mut img) => {
                    match bg_image {
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
                    if let Some(src) = bg_image {
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

            match (self.computed.outline_color, e.get_mut::<Outline>()) {
                (Some(color), Some(mut outline)) => {
                    outline.width = self.computed.outline_width;
                    outline.offset = self.computed.outline_offset;
                    outline.color = color;
                }
                (None, Some(_)) => {
                    e.remove::<Outline>();
                }
                (Some(color), None) => {
                    e.insert(Outline {
                        width: self.computed.outline_width,
                        offset: self.computed.outline_offset,
                        color,
                    });
                }
                (None, None) => {}
            }

            match (self.computed.z_index, e.get_mut::<ZIndex>()) {
                (Some(z), Some(_)) => {
                    e.insert(z);
                }
                (None, Some(_)) => {
                    e.remove::<ZIndex>();
                }
                (Some(zi), None) => {
                    e.insert(zi);
                }
                (None, None) => {}
            }

            match (self.computed.pickable, e.get_mut::<Pickable>()) {
                (Some(pe), Some(mut pickable)) => {
                    pickable.should_block_lower = pe == PointerEvents::All;
                    pickable.should_emit_events = pe == PointerEvents::All;
                }
                (None, Some(_)) => {
                    e.remove::<Pickable>();
                }
                (Some(pe), None) => {
                    e.insert(Pickable {
                        should_block_lower: pe == PointerEvents::All,
                        should_emit_events: pe == PointerEvents::All,
                    });
                }
                (None, None) => {}
            }

            let mut transform = Transform::default();
            transform.translation = self.computed.translation.unwrap_or(transform.translation);
            transform.scale.x = self.computed.scale_x.unwrap_or(1.);
            transform.scale.y = self.computed.scale_y.unwrap_or(1.);
            transform.rotate_z(self.computed.rotation.unwrap_or(0.));
            if is_animated_transform {
                let prev_transform = e.get_mut::<Transform>().unwrap().clone();
                let transition = self
                    .computed
                    .transitions
                    .iter()
                    .find(|t| t.property == TransitionProperty::Transform)
                    .unwrap();
                match e.get_mut::<AnimatedTransform>() {
                    Some(at) => {
                        if at.target.translation != transform.translation
                            || at.target.scale != transform.scale
                            || at.target.rotation != transform.rotation
                        {
                            e.insert(AnimatedTransform {
                                state: TransitionState {
                                    transition: transition.clone(),
                                    clock: 0.,
                                },
                                origin: prev_transform,
                                target: transform,
                            });
                        }
                    }
                    None => {
                        e.insert(AnimatedTransform {
                            state: TransitionState {
                                transition: transition.clone(),
                                clock: 0.,
                            },
                            origin: prev_transform,
                            target: transform,
                        });
                    }
                }
            } else {
                match e.get_mut::<Transform>() {
                    Some(tr) => {
                        if tr.translation != transform.translation
                            || tr.scale != transform.scale
                            || tr.rotation != transform.rotation
                        {
                            e.insert(transform);
                        }
                    }
                    None => {
                        panic!("Element has no transform!")
                    }
                }
            }
        }
    }
}
