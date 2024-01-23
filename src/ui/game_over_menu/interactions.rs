use bevy::prelude::*;

use crate::AppState;
use super::{layout, RestartEvent};

// Restart the game
pub fn restart_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<layout::RestartButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut restart_event_writer: EventWriter<RestartEvent>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match interaction {
            Interaction::Pressed => {restart_event_writer.send(RestartEvent); next_state.set(AppState::ResetGame)},
            _ => (), 
        }
    }
}