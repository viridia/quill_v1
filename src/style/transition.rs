use bevy::{prelude::*, ui, utils::HashMap};
use std::fmt::Debug;

/// Represents an animation timing function such as 'ease-in'.
pub trait TimingFunction
where
    Self: Send + Sync + Debug,
{
    fn eval(&self, t: f32) -> f32;
}

/// Module containing various useful timing functions.
pub mod timing {
    use std::{f32::consts::PI, fmt::Debug};

    use super::TimingFunction;

    /// Linear easing function
    pub struct Linear {}

    impl TimingFunction for Linear {
        fn eval(&self, t: f32) -> f32 {
            t
        }
    }

    impl Debug for Linear {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("linear")
        }
    }

    /// Cubic ease-in function
    pub struct EaseIn {}

    impl TimingFunction for EaseIn {
        fn eval(&self, t: f32) -> f32 {
            t * t * t
        }
    }

    impl Debug for EaseIn {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ease-in")
        }
    }

    /// Cubic ease-out function
    pub struct EaseOut {}

    impl TimingFunction for EaseOut {
        fn eval(&self, t: f32) -> f32 {
            1. - (1. - t).powf(3.)
        }
    }

    impl Debug for EaseOut {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ease-out")
        }
    }

    /// Sinusoidal ease-in-out function
    pub struct EaseInOut {}

    impl TimingFunction for EaseInOut {
        fn eval(&self, t: f32) -> f32 {
            -((PI * t).cos() - 1.) / 2.
        }
    }

    impl Debug for EaseInOut {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("ease-in-out")
        }
    }

    /// Linear easing function
    pub const LINEAR: &Linear = &Linear {};

    /// "ease-in" animation function
    pub const EASE_IN: &EaseIn = &EaseIn {};

    /// "ease-out" animation function
    pub const EASE_OUT: &EaseOut = &EaseOut {};

    /// "ease-in-out" animation function
    pub const EASE_IN_OUT: &EaseInOut = &EaseInOut {};
}

/// Specifies which property is being animated.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum TransitionProperty {
    /// Animate the element's transform
    Transform,

    /// Animate the element's background color
    BackgroundColor,

    /// Animate the element's border color
    BorderColor,

    /// Animate left
    Left,

    /// Animate top
    Top,

    /// Animate bottom
    Bottom,

    /// Animate right
    Right,

    /// Animate height
    Height,

    /// Animate width
    Width,

    /// Animate border left
    BorderLeft,

    /// Animate border top
    BorderTop,

    /// Animate border right
    BorderRight,

    /// Animate border bottom
    BorderBottom,
}

/// Defines a CSS-like animated transition
#[derive(Clone, Debug)]
pub struct Transition {
    /// Which property is to be animated.
    pub property: TransitionProperty,

    /// Delay before animation starts
    pub delay: f32,

    /// How long the animation should last
    pub duration: f32,

    /// Easing function
    pub timing: &'static dyn TimingFunction,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            property: TransitionProperty::Transform,
            delay: 0.,
            duration: 0.,
            timing: timing::LINEAR,
        }
    }
}

pub struct TransitionState {
    pub(crate) transition: Transition,
    // pub(crate) direction: f32,
    pub(crate) clock: f32,
}

impl TransitionState {
    pub fn advance(&mut self, delta: f32) {
        if self.transition.duration > 0. {
            self.clock = (self.clock + delta / self.transition.duration).clamp(0., 1.);
        } else {
            self.clock = 1.;
        }
    }

    // Return the current t parameter
    pub fn t(&self) -> f32 {
        self.transition.timing.eval(self.clock)
    }
}

#[derive(Component)]
#[doc(hidden)]
pub struct AnimatedTransform {
    pub(crate) state: TransitionState,
    pub(crate) origin: Transform,
    pub(crate) target: Transform,
}

#[derive(Component)]
#[doc(hidden)]
pub struct AnimatedBackgroundColor {
    pub(crate) state: TransitionState,
    pub(crate) origin: Color,
    pub(crate) target: Color,
}

#[derive(Component)]
#[doc(hidden)]
pub struct AnimatedBorderColor {
    pub(crate) state: TransitionState,
    pub(crate) origin: Color,
    pub(crate) target: Color,
}

pub struct AnimatedLayoutProp {
    pub(crate) state: TransitionState,
    pub(crate) origin: f32,
    pub(crate) target: f32,
}

impl AnimatedLayoutProp {
    pub fn new(state: TransitionState) -> Self {
        Self {
            state,
            origin: 0.,
            target: 0.,
        }
    }

    /// Update the [`Style`] component with the current animation value.
    pub fn update(&mut self, prop: TransitionProperty, style: &mut Style, delta: f32, force: bool) {
        let t_old = self.state.clock;
        self.state.advance(delta);
        let t = self.state.transition.timing.eval(self.state.clock);
        if t != t_old || force {
            let value = self.target * t + self.origin * (1. - t);
            match prop {
                TransitionProperty::Width => style.width = ui::Val::Px(value),
                TransitionProperty::Height => style.height = ui::Val::Px(value),
                TransitionProperty::Left => style.left = ui::Val::Px(value),
                TransitionProperty::Top => style.top = ui::Val::Px(value),
                TransitionProperty::Bottom => style.bottom = ui::Val::Px(value),
                TransitionProperty::Right => style.right = ui::Val::Px(value),
                TransitionProperty::BorderLeft => style.border.left = ui::Val::Px(value),
                TransitionProperty::BorderTop => style.border.top = ui::Val::Px(value),
                TransitionProperty::BorderRight => style.border.right = ui::Val::Px(value),
                TransitionProperty::BorderBottom => style.border.bottom = ui::Val::Px(value),
                TransitionProperty::Transform
                | TransitionProperty::BackgroundColor
                | TransitionProperty::BorderColor => panic!("Invalid style transition prop"),
            }
        }
    }

    /// Restart the animation with a new target if the target changed.
    pub fn restart_if_changed(
        &mut self,
        prop: TransitionProperty,
        prev_style: &Style, // The current style values
        next_style: &Style, // The targets we are going for
    ) {
        let (next, prev) = match prop {
            TransitionProperty::Width => (next_style.width, prev_style.width),
            TransitionProperty::Height => (next_style.height, prev_style.height),
            TransitionProperty::Left => (next_style.left, prev_style.left),
            TransitionProperty::Top => (next_style.top, prev_style.top),
            TransitionProperty::Bottom => (next_style.bottom, prev_style.bottom),
            TransitionProperty::Right => (next_style.right, prev_style.right),
            TransitionProperty::BorderLeft => (next_style.border.left, prev_style.border.left),
            TransitionProperty::BorderTop => (next_style.border.top, prev_style.border.top),
            TransitionProperty::BorderRight => (next_style.border.right, prev_style.border.right),
            TransitionProperty::BorderBottom => {
                (next_style.border.bottom, prev_style.border.bottom)
            }
            TransitionProperty::Transform
            | TransitionProperty::BackgroundColor
            | TransitionProperty::BorderColor => panic!("Invalid style transition prop"),
        };

        // Assume that all values are in pixels, we don't try and animate in other units.
        if let (ui::Val::Px(next_value), ui::Val::Px(prev_value)) = (next, prev) {
            if self.target != next_value {
                self.origin = prev_value;
                self.target = next_value;
                self.state.clock = 0.;
            }
        }
    }
}

#[derive(Component)]
#[doc(hidden)]
pub struct AnimatedLayout(pub HashMap<TransitionProperty, AnimatedLayoutProp>);

#[doc(hidden)]
pub fn animate_transforms(
    mut query: Query<(&mut Transform, &mut AnimatedTransform)>,
    time: Res<Time>,
) {
    for (mut trans, mut at) in query.iter_mut() {
        let t_old = at.state.clock;
        at.state.advance(time.delta_seconds());
        let t = at.state.transition.timing.eval(at.state.clock);
        if t != t_old {
            trans.scale = at.origin.scale.lerp(at.target.scale, t);
            trans.translation = at.origin.translation.lerp(at.target.translation, t);
            trans.rotation = at.origin.rotation.lerp(at.target.rotation, t);
        }
    }
}

#[doc(hidden)]
pub fn animate_bg_colors(
    mut query: Query<(
        Entity,
        Option<&mut BackgroundColor>,
        &mut AnimatedBackgroundColor,
    )>,
    time: Res<Time>,
) {
    #![allow(unused)]
    for (e, mut bg, mut at) in query.iter_mut() {
        let t_old = at.state.clock;
        at.state.advance(time.delta_seconds());
        let t = at.state.transition.timing.eval(at.state.clock);
        let origin = at.origin.as_rgba_linear();
        let target = at.target.as_rgba_linear();
        todo!("Finish color space interpolation!");
    }
}

#[doc(hidden)]
pub fn animate_border_colors(
    mut query: Query<(Entity, Option<&mut BorderColor>, &mut AnimatedBorderColor)>,
    time: Res<Time>,
) {
    #![allow(unused)]
    for (e, mut bg, mut at) in query.iter_mut() {
        let t_old = at.state.clock;
        at.state.advance(time.delta_seconds());
        let t = at.state.transition.timing.eval(at.state.clock);
        let origin = at.origin.as_rgba_linear();
        let target = at.target.as_rgba_linear();
        todo!("Finish color space interpolation!");
    }
}

#[doc(hidden)]
pub fn animate_layout(mut query: Query<(&mut Style, &mut AnimatedLayout)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for (mut style, mut anim) in query.iter_mut() {
        for (prop, trans) in anim.0.iter_mut() {
            trans.update(*prop, &mut style, delta, false);
        }
    }
}
