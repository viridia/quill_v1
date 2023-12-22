use bevy::{
    app::{App, Plugin},
    asset::{
        io::{file::FileAssetReader, AssetSource},
        AssetApp,
    },
};

/// Plugin which initializes all widgets and events.
pub struct GracklePlugin;

impl Plugin for GracklePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egret::EgretEventsPlugin)
            .register_asset_source(
                "grackle",
                AssetSource::build()
                    .with_reader(|| Box::new(FileAssetReader::new("crates/bevy_grackle/assets"))),
            );
    }
}
