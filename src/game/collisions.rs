use bevy::prelude::*;

use crate::{generic, art, game, AppState};

pub const PLAYER_HITBOX_RADIUS: f32 = 0.01;
pub const REMOVE_PLATFORM_X_PIXELS: f32 = 2.0; // How many pixels to remove from the edge of the platform hitboxes

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Collision>()
            .add_systems(Update, detect_collisions.run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}

#[derive(Event)]
pub struct Collision;

// Send a collision event when a collision is detected between the player and an obstacle
fn detect_collisions(
    platforms: Res<game::platforms::Platforms>,
    player_query: Query<&Transform, With<game::player::Player>>,
    screen_information: Res<generic::ScreenInformation>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;

        let player_hitbox = [
            generic::Range {min: player_translation.x - PLAYER_HITBOX_RADIUS, max: player_translation.x + PLAYER_HITBOX_RADIUS}, // X
            generic::Range {min: player_translation.y - PLAYER_HITBOX_RADIUS, max: player_translation.y + PLAYER_HITBOX_RADIUS}, // Y
        ];

        let mut collision = false;

        // Detect if player is colliding with the wall
        if player_hitbox[0].max > screen_information.window_width - screen_information.x_deadspace {
            collision = true;
        } else if player_hitbox[0].min < screen_information.x_deadspace {
            collision = true;
        }

        // Detect if player is colliding with any existing platforms
        for platform in platforms.platforms_vec.iter() {
            let x_collision = match platform.side {
                generic::Direction::Left => player_hitbox[0].min < platform.hitbox[0].max - REMOVE_PLATFORM_X_PIXELS * art::SPRITE_SCALE as f32,
                generic::Direction::Right => player_hitbox[0].max > platform.hitbox[0].max + REMOVE_PLATFORM_X_PIXELS * art::SPRITE_SCALE as f32,
            };

            let y_collision = player_hitbox[1].min < platform.hitbox[1].max && player_hitbox[1].max > platform.hitbox[1].min;

            // Only update the collision if the player hasn't allready collided with the wall
            if !collision {
                collision = x_collision & y_collision;
            }
            
        }

        // When there is a collision change app to game over state
        if collision {
            next_state.set(AppState::GameOver);
        }
    }    
}
