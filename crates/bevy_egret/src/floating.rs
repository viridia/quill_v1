use bevy::{
    app::{App, Plugin, PostUpdate},
    ecs::{component::Component, entity::Entity, query::Without, system::Query},
    math::Rect,
    transform::components::GlobalTransform,
    ui::{self, Node, Style},
    window::Window,
};

/// Which side of the anchor the floating element should be placed.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FloatSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// How the floating element should be aligned to the anchor.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum FloatAlign {
    #[default]
    Start,
    End,
    Center,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FloatPosition {
    /// The side of the anchor the floating element should be placed.
    pub side: FloatSide,

    /// How the floating element should be aligned to the anchor.
    pub align: FloatAlign,

    /// If true, the floating element will be at least as large as the anchor on the adjacent
    /// side.
    pub stretch: bool,

    /// The gap between the anchor and the floating element.
    pub gap: f32,
}

#[derive(Component)]
pub struct Floating {
    /// The entity that this floating element is anchored to.
    pub anchor: Entity,

    /// The position of the floating element relative to the anchor.
    pub position: Vec<FloatPosition>,
}

impl Clone for Floating {
    fn clone(&self) -> Self {
        Self {
            anchor: self.anchor,
            position: self.position.clone(),
        }
    }
}

pub fn position_floating(
    mut query: Query<(&mut Style, &Floating, &GlobalTransform)>,
    anchor_query: Query<(&Node, &GlobalTransform), Without<Floating>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let ww = window.resolution.physical_width() as f32;
    let wh = window.resolution.physical_height() as f32;
    let sf = window.resolution.scale_factor() as f32;

    let window_rect = Rect::new(0., 0., ww / sf, wh / sf).inset(8.);

    for (mut style, floating, floating_transform) in query.iter_mut() {
        let Ok((anchor, anchor_transform)) = anchor_query.get(floating.anchor) else {
            continue;
        };

        let anchor_rect = anchor.logical_rect(anchor_transform);
        let mut best_occluded = f32::MAX;
        let mut best_rect = Rect::default();
        let mut best_position: FloatPosition = Default::default();

        for position in &floating.position {
            let floating_rect = anchor.logical_rect(floating_transform);
            let mut rect = Rect::default();

            // Taraget width and height depends on whether 'stretch' is true.
            let target_width = if position.stretch && position.side == FloatSide::Top
                || position.side == FloatSide::Bottom
            {
                floating_rect.width().max(anchor_rect.width())
            } else {
                floating_rect.width()
            };

            let target_height = if position.stretch && position.side == FloatSide::Left
                || position.side == FloatSide::Right
            {
                floating_rect.height().max(anchor_rect.height())
            } else {
                floating_rect.height()
            };

            // Position along main axis.
            match position.side {
                FloatSide::Top => {
                    rect.max.y = anchor_rect.min.y - position.gap;
                    rect.min.y = rect.max.y - floating_rect.height();
                }

                FloatSide::Bottom => {
                    rect.min.y = anchor_rect.max.y + position.gap;
                    rect.max.y = rect.min.y + floating_rect.height();
                }

                FloatSide::Left => {
                    rect.max.x = anchor_rect.min.x - position.gap;
                    rect.min.x = rect.max.x - floating_rect.width();
                }

                FloatSide::Right => {
                    rect.min.x = anchor_rect.max.x + position.gap;
                    rect.max.x = rect.min.x + floating_rect.width();
                }
            }

            // Position along secondary axis.
            match position.align {
                FloatAlign::Start => match position.side {
                    FloatSide::Top | FloatSide::Bottom => {
                        rect.min.x = anchor_rect.min.x;
                        rect.max.x = rect.min.x + target_width;
                    }

                    FloatSide::Left | FloatSide::Right => {
                        rect.min.y = anchor_rect.min.y;
                        rect.max.y = rect.min.y + target_height;
                    }
                },

                FloatAlign::End => match position.side {
                    FloatSide::Top | FloatSide::Bottom => {
                        rect.max.x = anchor_rect.max.x;
                        rect.min.x = rect.max.x - target_width;
                    }

                    FloatSide::Left | FloatSide::Right => {
                        rect.max.y = anchor_rect.max.y;
                        rect.min.y = rect.max.y - target_height;
                    }
                },

                FloatAlign::Center => match position.side {
                    FloatSide::Top | FloatSide::Bottom => {
                        rect.min.x = (anchor_rect.width() - target_width) * 0.5;
                        rect.max.x = rect.min.x + target_width;
                    }

                    FloatSide::Left | FloatSide::Right => {
                        rect.min.y = (anchor_rect.width() - target_height) * 0.5;
                        rect.max.y = rect.min.y + target_height;
                    }
                },
            }

            // Clip to window and see how much of the floating element is occluded.
            let clipped_rect = floating_rect.intersect(window_rect);
            let occlusion = floating_rect.width() * floating_rect.height()
                - clipped_rect.width() * clipped_rect.height();

            // Find the position that has the least occlusion.
            if occlusion < best_occluded {
                best_occluded = occlusion;
                best_rect = rect;
                best_position = *position;
            }
        }

        if best_occluded < f32::MAX {
            style.left = ui::Val::Px(best_rect.min.x);
            style.top = ui::Val::Px(best_rect.min.y);
            if best_position.stretch {}
        }
    }
}

pub struct EgretFloatingPlugin;

impl Plugin for EgretFloatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, position_floating);
    }
}
