use bevy::prelude::*;
use crate::ui::{styles, helpers};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        
        (
            Hud,

            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    

                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    ..default()
                },
                ..default()
            }
        )
    )
    
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(1.0), // Score text is spaced 1% from the top of the screen
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }
        );

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