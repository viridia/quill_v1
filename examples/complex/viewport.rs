use bevy::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

/// Marker which identifies the primary camera.
#[derive(Component)]
pub struct PrimaryCamera;

/// Used to create margins around the viewport so that side panels don't overwrite the 3d scene.
#[derive(Default, Resource, PartialEq)]
pub struct ViewportInset {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

/// A marker component for that identifies which element contains the 3d view. The
/// `update_viewport_inset` system measures the on-screen position of the UiNode that this
/// component is attached to, and updates the screen position of the 3D view to match it.
#[derive(Component, Clone)]
pub struct ViewportInsetElement;
