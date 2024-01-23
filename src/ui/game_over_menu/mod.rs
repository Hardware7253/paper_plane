use bevy::prelude::*;

use crate::AppState;

pub mod layout;
pub mod interactions;

#[derive(Event)]
pub struct RestartEvent;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RestartEvent>()
            .add_systems(OnEnter(AppState::GameOver), layout::spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), layout::despawn_game_over_menu)
            .add_systems(Update, reset_game.run_if(in_state(AppState::ResetGame)))
            .add_systems(Update, interactions::restart_button_interactions.run_if(in_state(AppState::GameOver)));
    }
}

fn reset_game(
    mut next_state: ResMut<NextState<AppState>>,
    mut restart: EventReader<RestartEvent>,
) {
    for _ in restart.read() {
        next_state.set(AppState::Game);
    }
}

