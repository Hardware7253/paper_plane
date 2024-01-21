use bevy::prelude::*;

use crate::AppState;

pub mod layout;
pub mod interactions;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), layout::spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), layout::despawn_game_over_menu)
            .add_systems(Update, interactions::restart_button_interactions.run_if(in_state(AppState::GameOver)));
    }
}