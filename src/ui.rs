
mod sidebar;
mod game_screen;

use bevy::prelude::*;

pub struct Render;

impl Plugin for Render {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_systems(Startup, game_screen::render);
    }
}

