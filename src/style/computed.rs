use super::style_props::PointerEvents;
use super::transition::{
    AnimatedBackgroundColor, AnimatedBorderColor, AnimatedLayout, AnimatedLayoutProp,
    AnimatedTransform, Transition, TransitionProperty, TransitionState,
};
use bevy::asset::AssetPath;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy::ui::widget::UiImageSize;
use bevy::ui::ContentSize;
use bevy::utils::HashMap;
use bevy_mod_picking::prelude::Pickable;

/// A computed style represents the composition of one or more `ElementStyle`s.
#[derive(Default, Clone, Debug)]
#[doc(hidden)]
pub struct ComputedStyle {
    pub style: Style,

    // Text properties
    pub alignment: Option<JustifyText>,
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    pub font: Option<AssetPath<'static>>,
    pub font_handle: Option<Handle<Font>>,
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
    pub image_handle: Option<Handle<Image>>,
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
}

/// Custom command that updates the style of an entity.
pub struct UpdateComputedStyle {
    pub(crate) entity: Entity,
    pub(crate) computed: ComputedStyle,
}

impl Command for UpdateComputedStyle {
    fn apply(self, world: &mut World) {
        let Some(mut e) = world.get_entity_mut(self.entity) else {
            return;
        };

        let mut is_animated_bg_color = false;
        let mut is_animated_border_color = false;
        let mut is_animated_transform = false;
        let mut is_animated_layout = false;

        let mut next_style = self.computed.style;

        self.computed
            .transitions
            .iter()
            .for_each(|tr| match tr.property {
                TransitionProperty::Transform => is_animated_transform = true,
                TransitionProperty::BackgroundColor => is_animated_bg_color = true,
                TransitionProperty::BorderColor => is_animated_border_color = true,
                TransitionProperty::Height
                | TransitionProperty::Width
                | TransitionProperty::Left
                | TransitionProperty::Top
                | TransitionProperty::Bottom
                | TransitionProperty::Right
                | TransitionProperty::BorderLeft
                | TransitionProperty::BorderTop
                | TransitionProperty::BorderRight
                | TransitionProperty::BorderBottom => is_animated_layout = true,
            });

        let bg_image = self.computed.image_handle;

        // If any layout properties are animated, insert animation components and mutate
        // the style that's going to get inserted
        if is_animated_layout {
            // Get the current style
            let prev_style: Style = match e.get::<Style>() {
                Some(st) => st.clone(),
                None => next_style.clone(),
            };

            // TODO: Make sure the set of transitions hasn't changed.

            // If there's already animations
            if let Some(mut anim) = e.get_mut::<AnimatedLayout>() {
                for (prop, trans) in anim.0.iter_mut() {
                    trans.restart_if_changed(*prop, &prev_style, &next_style);
                    trans.update(*prop, &mut next_style, 0., true);
                }
            } else {
                let mut anim =
                    AnimatedLayout(HashMap::with_capacity(self.computed.transitions.len()));
                self.computed
                    .transitions
                    .iter()
                    .for_each(|tr| match tr.property {
                        TransitionProperty::Left
                        | TransitionProperty::Top
                        | TransitionProperty::Right
                        | TransitionProperty::Bottom
                        | TransitionProperty::Height
                        | TransitionProperty::Width
                        | TransitionProperty::BorderLeft
                        | TransitionProperty::BorderTop
                        | TransitionProperty::BorderRight
                        | TransitionProperty::BorderBottom => {
                            let mut ap = AnimatedLayoutProp::new(TransitionState {
                                transition: tr.clone(),
                                clock: 0.,
                            });
                            ap.update(tr.property, &mut next_style, 0., true);
                            anim.0.insert(tr.property, ap);
                        }
                        _ => (),
                    });
                e.insert(anim);
            }
        }

        if let Some(mut existing_style) = e.get_mut::<Style>() {
            // Update the existing style
            if !existing_style.eq(&next_style) {
                *existing_style = next_style;
            }
        } else {
            // Insert a new Style component
            e.insert(next_style);
        }

        if let Some(mut text) = e.get_mut::<Text>() {
            // White is the default.
            let color = self.computed.color.unwrap_or(Color::WHITE);
            for section in text.sections.iter_mut() {
                if section.style.color != color {
                    section.style.color = color;
                }
            }

            if let Some(ws) = self.computed.line_break {
                if text.linebreak_behavior != ws {
                    text.linebreak_behavior = ws;
                }
            }

            if let Some(font_size) = self.computed.font_size {
                for section in text.sections.iter_mut() {
                    if section.style.font_size != font_size {
                        section.style.font_size = font_size;
                    }
                }
            }

            if let Some(ref font) = self.computed.font_handle {
                for section in text.sections.iter_mut() {
                    if section.style.font != *font {
                        section.style.font = font.clone();
                    }
                }
            }
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
                    if self.computed.background_color.is_some() {
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
                    if self.computed.border_color.is_some() {
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

        // Update outline
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

        // Update Z-Index
        match (self.computed.z_index, e.get::<ZIndex>()) {
            // Don't change if value is the same
            (Some(ZIndex::Local(zi)), Some(ZIndex::Local(zo))) if zi == *zo => {}
            (Some(ZIndex::Global(zi)), Some(ZIndex::Global(zo))) if zi == *zo => {}
            (Some(zi), Some(_)) => {
                e.insert(zi);
            }
            (None, Some(_)) => {
                e.remove::<ZIndex>();
            }
            (Some(zi), None) => {
                e.insert(zi);
            }
            (None, None) => {}
        }

        // Update Pickable
        match (self.computed.pickable, e.get_mut::<Pickable>()) {
            (Some(pe), Some(mut pickable)) => {
                pickable.should_block_lower = pe == PointerEvents::All;
                pickable.is_hoverable = pe == PointerEvents::All;
            }
            (None, Some(_)) => {
                e.remove::<Pickable>();
            }
            (Some(pe), None) => {
                e.insert(Pickable {
                    should_block_lower: pe == PointerEvents::All,
                    is_hoverable: pe == PointerEvents::All,
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
            let prev_transform = *e.get_mut::<Transform>().unwrap();
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
                        origin: transform,
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
