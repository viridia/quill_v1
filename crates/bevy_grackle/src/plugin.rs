use bevy::app::{App, Plugin};

/// Plugin which initializes all widgets and events.
pub struct GracklePlugin;

impl Plugin for GracklePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egret::EgretEventsPlugin);
    }
}
