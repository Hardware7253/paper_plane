use bevy::prelude::*;
use crate::ui::{styles, helpers};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        // Main menu background, and parent node
        (
            Hud,

            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    width: Val::Percent(100.0),
                    height: Val::Percent(12.0),

                    ..default()
                },
                ..default()
            }
        )
    )

    .with_children(|parent| {
        parent.spawn(
            (
                ScoreText,
                helpers::text("0", styles::TITLE_TEXT_STYLE, &asset_server)
            )
        );
    });
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}