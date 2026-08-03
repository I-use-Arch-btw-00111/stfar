
mod assets;
mod ui;
mod utils;

use bevy::{
    log::LogPlugin, prelude::*, window::{PresentMode, WindowResolution, WindowTheme}
};

const APP_ID: &str = "stfar";
const APP_TITLE: &str = "STFAR - Some Touhou Fangame Attempt in Rust";
const WINDOW_MIN_SIZE: (u32, u32) = (1248, 720);

fn main() {
    let window = Window {
        title: APP_TITLE.to_owned(),
        name: APP_ID.to_owned().into(),
        present_mode: PresentMode::AutoVsync,
        resolution: WindowResolution::new(WINDOW_MIN_SIZE.0, WINDOW_MIN_SIZE.1),
        window_theme: Some(WindowTheme::Dark),
        position: WindowPosition::Centered(MonitorSelection::Primary),
        resize_constraints: WindowResizeConstraints {
            min_width: WINDOW_MIN_SIZE.0 as f32,
            min_height: WINDOW_MIN_SIZE.1 as f32,
            ..default()
        },
        ..default()
    };
    let window_plugin = WindowPlugin {
        primary_window: Some(window),
        ..default()
    };
    let log_plugin = LogPlugin {
        filter: "info,stfar=trace".to_owned(),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin).set(log_plugin))
        .add_plugins(assets::LoadAssets)
        .add_plugins(ui::Render)
        .run();
}
