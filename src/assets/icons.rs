
use bevy::{log, prelude::*};

#[derive(Resource)]
pub struct IconAssets {
    pub heart: Handle<Image>,
    pub bomb: Handle<Image>
}

pub(super) fn load_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(IconAssets {
        heart: asset_server.load("icons/heart.png"),
        bomb: asset_server.load("icons/bomb.png"),
    });
    log::info!("Loaded icons");
}

