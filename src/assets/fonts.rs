
use bevy::{log, prelude::*};

#[derive(Resource)]
pub struct FontAssets {
    pub primary: Handle<Font>,
}

impl FontAssets {
    pub fn primary(&self) -> FontSource {
        self.primary.clone().into()
    }
}

pub(super) fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontAssets {
        primary: asset_server.load("fonts/GermaniaOne.ttf"),
    });
    log::info!("Loaded fonts");
}

