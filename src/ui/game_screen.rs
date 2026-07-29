
use bevy::{camera::{Viewport, visibility::RenderLayers}, prelude::*};
use crate::assets::{FontAssets, IconAssets};
use super::sidebar;

pub(super) fn render(mut cmds: Commands, fonts: Res<FontAssets>, icons: Res<IconAssets>) {
    let ui_camera = render_ui_camera(&mut cmds);
    render_gameplay_screen(&mut cmds);
    sidebar::render(&mut cmds, ui_camera, &fonts, &icons);
}

fn render_ui_camera(cmds: &mut Commands) -> Entity {
    cmds.spawn((
        Camera2d,
        Camera { order: 1, clear_color: ClearColorConfig::None, ..default() },
        RenderLayers::layer(1)
    )).id()
}

fn render_gameplay_screen(cmds: &mut Commands) {
    let viewport = Viewport {
        physical_position: UVec2::new(32, 34),
        physical_size: UVec2::new(892, 652),
        ..default()
    };

    cmds.spawn((
        Camera2d,
        Camera {
            order: 0,
            viewport: Some(viewport),
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(0)
    ));

    cmds.spawn((
         Sprite {
            color: Color::srgb(0.1, 0.15, 0.3),
            custom_size: Some(Vec2::new(892.0, 652.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        RenderLayers::layer(0),
    ));
}

