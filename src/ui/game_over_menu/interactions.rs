use bevy::prelude::*;

use crate::AppState;
use super::layout;

// Unpause the game
pub fn restart_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match interaction {
            Interaction::Pressed => next_state.set(AppState::Game),
            _ => (), 
        }
    }
}