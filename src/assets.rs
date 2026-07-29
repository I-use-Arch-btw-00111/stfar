
mod fonts;
mod icons;

use bevy::{log, prelude::*};
pub use fonts::FontAssets;
pub use icons::IconAssets;

pub struct LoadAssets;

impl Plugin for LoadAssets {
    fn build(&self, app: &mut App) {
        log::debug!("Loading assets");
        app.add_systems(PreStartup, fonts::load_fonts);
        app.add_systems(PreStartup, icons::load_icons);
    }
}
