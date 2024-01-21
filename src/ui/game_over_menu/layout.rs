use bevy::prelude::*;

use crate::ui;
use ui::{helpers, styles};

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct RestartButton;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(

        (
            GameOverMenu,

            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,

                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    row_gap: Val::Percent(1.0),

                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            }
        )
    )
    
    .with_children(|parent| {

        // Title
        parent.spawn(
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            parent.spawn(
                helpers::text("Game Over", styles::TITLE_TEXT_STYLE, &asset_server)
            );
        });
        
        // Restart game button
        parent.spawn(
            (
                ui::GenericButton,
                RestartButton,
                helpers::button(styles::BUTTON_STYLE),
            )   
        )
        .with_children(|parent| {
            parent.spawn(
                helpers::text("Restart", styles::BUTTON_STYLE.text_style.unwrap(), &asset_server) // Play button text
            );
        });

        // Back to main menu button
        parent.spawn(
            (
                ui::GenericButton,
                ui::GenericBackButton,
                helpers::button(styles::BUTTON_STYLE),
            )   
        )
        .with_children(|parent| {
            parent.spawn(
                helpers::text("Main Menu", styles::BUTTON_STYLE.text_style.unwrap(), &asset_server) // Play button text
            );
        });

    });
}

pub fn despawn_game_over_menu(mut commands: Commands, hud_query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}