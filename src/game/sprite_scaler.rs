use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

pub struct SpriteScalerPlugin;

use crate::art;

#[derive(Resource, Debug)]
pub struct ScaleFactor {
    pub current: f32, // Used for spawning sprites during the current instance of the game
}

impl Plugin for SpriteScalerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScaleFactor{current: 0.0})
            .add_systems(PreStartup, calcualte_sprite_scale)
            .add_systems(Update, calcualte_sprite_scale);
    }
}

// Calculates sprite scale factor by making the level as wide as possible without exceeding the edges of the screen
fn calcualte_sprite_scale(
    mut resize_event: EventReader<WindowResized>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut scale_factor: ResMut<ScaleFactor>,
) {
    for _ in resize_event.read() {
        if let Ok(window) = window_query.get_single() {
            
            let unscaled_height = art::WALL_SPRITE_SIZE.x * 5.0; // The desired (unscaled) height of the level, in this case 5 wall sprites

            scale_factor.current = (window.height() / unscaled_height).floor(); // Set the scale factor so that the level is 5 wall sprites high
        }
    }
}