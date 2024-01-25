use bevy::prelude::*;
use crate::AppState;
use crate::game;

pub mod layout;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), layout::spawn_hud)

            .add_systems(OnExit(game::GameState::GameOver), layout::despawn_hud)
            .add_systems(OnEnter(AppState::MainMenu), layout::despawn_hud)

            .add_systems(Update, update_hud_score.run_if(in_state(AppState::Game)));
    }
}

fn update_hud_score(
    mut text_query: Query<&mut Text, With<layout::ScoreText>>,
    game: Res<game::Game>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = game.score.to_string();
    }
}