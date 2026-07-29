
use bevy::{prelude::*, ecs::relationship::RelatedSpawnerCommands};
use crate::assets::{FontAssets, IconAssets};

pub(super) fn render(
    cmds: &mut Commands,
    ui_camera: Entity,
    fonts: &FontAssets,
    icons: &IconAssets
) {
    let font = TextFont {
        font: fonts.primary(),
        font_size: FontSize::Px(28.0),
        ..default()
    };
    let color = TextColor(Color::WHITE);

    let padding = UiRect {
        left: Val::Px(20.0),
        right: Val::Px(20.0),
        top: Val::Px(30.0),
        bottom: Val::Px(30.0),
    };
    let container = Node {
        padding,
        position_type: PositionType::Absolute,
        right: Val::Px(0.0),
        top: Val::Px(0.0),
        width: Val::Px(356.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexStart,
        row_gap: Val::Px(25.0), 
        ..default()
    };

    cmds
        .spawn((container, UiTargetCamera(ui_camera)))
        .with_children(|parent| {
            render_score(parent, font.clone(), color);
            render_health_row(parent, icons, font.clone(), color);
            render_bomb_row(parent, icons, font.clone(), color);
            render_spacer(parent);
            render_stage_timer(parent, font.clone(), color);
            render_overall_timer(parent, font.clone(), color);
        });
}

fn render_spacer(parent: &mut RelatedSpawnerCommands<ChildOf>) {
    parent.spawn(Node { height: Val::Px(50.0), ..default()});
}

fn render_score(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: TextFont,
    color: TextColor
) {
    parent.spawn((Text::new("Score: 000000000"), font, color));
}

fn render_stage_timer(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: TextFont,
    color: TextColor
) {
    parent.spawn((Text::new("Stage Time: 00:00"), font, color));
}

fn render_overall_timer(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: TextFont,
    color: TextColor
) {
    parent.spawn((Text::new("Total Time: 00:00"), font, color));
}

fn render_health_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    icons: &IconAssets,
    font: TextFont,
    color: TextColor
) {
    let container = Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    };
    let text_row = Node { width: Val::Px(110.0), ..default() };
    let icon_row = Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(6.0),
        ..default()
    };

    parent.spawn(container).with_children(|row| {
        row.spawn((Text::new("Health:"), font, color, text_row));
        row.spawn(icon_row).with_children(|container| {
            render_icons(container, &icons.heart)
        });
    });
}

fn render_bomb_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    icons: &IconAssets,
    font: TextFont,
    color: TextColor
) {
    let container = Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    };
    let text_row = Node { width: Val::Px(110.0), ..default() };
    let icon_row = Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(6.0),
        ..default()
    };

    parent.spawn(container).with_children(|row| {
        row.spawn((Text::new("Bombs:"), font.to_owned(), color.to_owned(), text_row));
        row.spawn(icon_row).with_children(|container| {
            render_icons(container, &icons.bomb)
        });
    });
}

fn render_icons(container: &mut RelatedSpawnerCommands<ChildOf>, icon: &Handle<Image>) {
    const ICON_COUNT: u8 = 8;

    for _ in 0..ICON_COUNT {
        let image_slot = Node {
            height: Val::Px(20.0),
            width: Val::Px(20.0),
            ..default()
        };
        container.spawn((
            ImageNode::new(icon.clone()),
            image_slot
        ));
    }
}

