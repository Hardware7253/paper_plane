use bevy::prelude::*;

use crate::{generic, game, AppState};
use game::sprite_scaler;

pub const REMOVE_PLATFORM_X_PIXELS: f32 = 2.0; // How many pixels to remove from the edge of the platform hitboxes

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Collision>()
            .add_systems(Update, detect_collisions.after(generic::update_screen_information).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}

#[derive(Event)]
pub struct Collision;

// Send a collision event when a collision is detected between the player and an obstacle
fn detect_collisions(
    platforms: Res<game::platforms::Platforms>,
    player_query: Query<&Transform, With<game::player::Player>>,
    screen_information: Res<generic::ScreenInformation>,
    mut next_state: ResMut<NextState<game::GameState>>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;

        let mut collision = false;

        // Detect if player is colliding with the wall
        if player_translation.x > screen_information.window_width - screen_information.x_deadspace {
            collision = true;
        } else if player_translation.x < screen_information.x_deadspace {
            collision = true;
        }

        //println!("wall collision: {}, {}, {}, {}", collision, screen_information.window_width - screen_information.x_deadspace, screen_information.x_deadspace, player_translation.x);

        // Detect if player is colliding with any existing platforms
        for platform in platforms.platforms_vec.iter() {
            let x_collision = match platform.side {
                generic::Direction::Left => player_translation.x < platform.hitbox[0].max - REMOVE_PLATFORM_X_PIXELS * scale_factor.current,
                generic::Direction::Right => player_translation.x > platform.hitbox[0].max + REMOVE_PLATFORM_X_PIXELS * scale_factor.current,
            };

            let y_collision = player_translation.y < platform.hitbox[1].max && player_translation.y > platform.hitbox[1].min;

            // Only update the collision if the player hasn't allready collided with the wall
            if !collision {
                collision = x_collision & y_collision;
            }
            
        }

        // When there is a collision change app to game over state
        if collision {
            next_state.set(game::GameState::GameOver);
        }
    }    
}
