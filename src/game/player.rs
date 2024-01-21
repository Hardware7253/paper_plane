use bevy::prelude::*;
use std::time::Instant;
use crate::{art, generic, game, AppState};
use generic::Direction;

const START_DIRECTION: Direction = Direction::Right; // Direction player starts facing
const AUTO_MOVE_MS: u8 = 50; // Millesconds which a button must be held down for inorder for the player to start cycling through the sprite sheet
const DEATH_ANIMATION_FPS: f32 = 8.0;
//const PLAYER_SPAWN_X: f32 = 

#[derive(Component, Debug)]
pub struct Player {
    pub speed: Vec2, // X and y speed

    facing: Direction, // Direction the player is currently facing
    sprite_sheet_index: usize, // Current index for the player sprite sheet

    // This is information is used to accelerate the rate at which the player turns while a movement button is being held
    last_button_press: Option<Instant>, // Time when the last button (started) being pressed
    last_frame_requested_direction: Option<Direction>, // This updates with what button the player is pressing every frame
    force_update_direction: Option<Direction>, // Acts the same as a keypress, changing the players direction in the next frame
}

#[derive(Component)]
struct DeathAnimation;

#[derive(Resource)]
struct DeathAnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DeathAnimationTimer(Timer::from_seconds(1.0 / DEATH_ANIMATION_FPS, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(Update, animate_death.run_if(in_state(AppState::GameOver)))
            .add_systems(Update, (change_player_heading, move_player).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}


// Spawns player and initializes death animation
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    screen_infromation: Res<generic::ScreenInformation>
) {
    let player_texture_atlas = TextureAtlas::from_grid(
        asset_server.load(art::PLAYER_SPRITE_SHEET_PATH),
        art::PLAYER_SPRITE_SIZE,
        art::PLAYER_SPRITE_SHEET_COLUMNS,
        art::PLAYER_SPRITE_SHEET_ROWS,
        None,
        None, 
    );

    let death_texture_atlas = TextureAtlas::from_grid(
        asset_server.load(art::DEATH_SPRITE_SHEET_PATH),
        art::DEATH_SPRITE_SIZE,
        art::DEATH_SPRITE_SHEET_COLUMNS,
        art::DEATH_SPRITE_SHEET_ROWS,
        None,
        None, 
    );

    // Spawn player with their back against the wall
    let player_spawn_x = match START_DIRECTION {
        Direction::Left => screen_infromation.window_width - screen_infromation.x_deadspace - art::PLAYER_WORLD_SIZE.x / 2.0,
        Direction::Right => screen_infromation.x_deadspace + art::PLAYER_WORLD_SIZE.x / 2.0,
    };

    // Spawn player
    commands.spawn(
        (
            game::GameComponent,
            Player {
                speed: Vec2::new(0.0, 0.0),
                facing: START_DIRECTION,
                sprite_sheet_index: art::PLAYER_SPRITE_SHEET_START_INDEX,
                last_button_press: None,
                last_frame_requested_direction: None,
                force_update_direction: None,
            },

            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(player_texture_atlas),
                sprite: TextureAtlasSprite::new(art::PLAYER_SPRITE_SHEET_START_INDEX),
                transform: Transform {
                    translation: Vec3::new(player_spawn_x, screen_infromation.window_height / 2.0, 2.0),
                    scale: Vec3::splat(art::SPRITE_SCALE as f32),
                    ..default()
                },
                ..default()
            },
        )
    );
    
    // Spawn death animation
    commands.spawn(
        (
            game::GameComponent,
            DeathAnimation,

            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(death_texture_atlas),
                sprite: TextureAtlasSprite::new(0),
                visibility: Visibility::Hidden,
                transform: Transform {
                    translation: Vec3::splat(2.0), // The location of the death animation isn't important, as long as it is above other sprites on the z axis
                    scale: Vec3::splat(art::SPRITE_SCALE as f32),
                    ..default()
                },
                ..default()
            },
        )
    );
}

// Changes player heading with arrow keys
fn change_player_heading(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite)>,
    game: Res<game::Game>,
) {
    if let Ok((player, sprite)) = &mut player_query.get_single_mut() {
        let mut requested_index: i32 = sprite.index as i32;
        let dir_multiplier: i32;

        // Flip sprite direction if the player is facing right
        if player.facing == Direction::Right {
            dir_multiplier = 1;
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
            dir_multiplier = -1;
        }

        // Increment / deincrement sprite sheet index to rotate sprite when the player presses the direction keys
        if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right) || player.force_update_direction == Some(Direction::Right) {
            requested_index -= 1 * dir_multiplier;
            player.last_button_press = Some(Instant::now());
        }
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) || player.force_update_direction == Some(Direction::Left) {
            requested_index += 1 * dir_multiplier;
            player.last_button_press = Some(Instant::now());
        }
        player.force_update_direction = None;

        // If the sprite index goes out of range the sprite should be flipped, so it can rotate back in the other direction
        if requested_index >= art::PLAYER_SPRITESHEET_INDICES as i32 {
            player.facing = player.facing.reverse();
            sprite.flip_x ^= true;
            sprite.index -= 1;
        } else if requested_index >= 0 {
            sprite.index = requested_index as usize;
            player.sprite_sheet_index = requested_index as usize;
        }

        // Update player speed with (potentially) new index
        let speed_steps = Vec2::new(game.difficulty.player_max_speed.x / (art::PLAYER_SPRITESHEET_INDICES - 1) as f32, game.difficulty.player_max_speed.y / art::PLAYER_SPRITESHEET_INDICES as f32);
        player.speed = calculate_speed(sprite.index, speed_steps);

        // Get current key press direction
        let mut key_press_direction: Option<Direction> = None;
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            key_press_direction = Some(Direction::Right);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            key_press_direction = Some(Direction::Left);
        }

        // Update / reset button information
        if player.last_frame_requested_direction == None {
            player.last_frame_requested_direction = key_press_direction;
        } else {
            if key_press_direction == None {
                player.last_button_press = None;
                player.last_frame_requested_direction = None;
            }
            if key_press_direction != player.last_frame_requested_direction {
                player.last_frame_requested_direction = key_press_direction;
            }
        }

        // Start automatically changing the player heading once a direction key has been held for a certain ammount of time
        if let Some(press_time) = player.last_button_press {

            // AUTO_MOVE_MS * 2 when on the last sprite in the player spritesheet
            // Done to make it easier to stop in this position
            let mut auto_move_ms = AUTO_MOVE_MS;
            if sprite.index == art::PLAYER_SPRITESHEET_INDICES - 1 {
                auto_move_ms *= 2;
            }

            if press_time.elapsed().as_millis() > auto_move_ms as u128 {
                player.force_update_direction = player.last_frame_requested_direction;
            }
        }
    }
}

// Moves players x coordinate every frame according to player speed
pub fn move_player(mut player_query: Query<(&mut Transform, &Player)>, time: Res<Time>) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
        let x_direction: f32;
        if player.facing == Direction::Right {
            x_direction = 1.0;
        } else {
            x_direction = -1.0;
        }
    
        transform.translation.x += player.speed.x * x_direction * time.delta_seconds();
        transform.translation.y -= player.speed.y * time.delta_seconds();
    }
}

// Calculate speed based on player spritesheet index
fn calculate_speed(index: usize, speed_steps: Vec2) -> Vec2 {
    let reverse_index = (index as i32 - (art::PLAYER_SPRITESHEET_INDICES - 1) as i32).abs() as usize;
    let indices = (reverse_index, index);

    Vec2::new(speed_steps.x * (indices.0) as f32, speed_steps.y * (indices.1 + 1) as f32)
}

// Play the death animation when the player dies
fn animate_death(
    mut commands: Commands,
    player_query: Query<(&Transform, Entity), (With<Player>, Without<DeathAnimation>)>,
    mut death_animation_query: Query<(&mut TextureAtlasSprite, Entity, &mut Visibility, &mut Transform), With<DeathAnimation>>,
    mut animation_timer: ResMut<DeathAnimationTimer>,
    time: Res<Time>,
) {
    if let Ok((mut death_sprite, death_entity, mut death_visibility, mut death_transform))  = death_animation_query.get_single_mut() {
        *death_visibility = Visibility::Visible;

        // Move the animation to where the player died, and despawn the player
        if let Ok((player_transform, player_entity)) = player_query.get_single() {
            death_transform.translation = player_transform.translation;
            commands.entity(player_entity).despawn();
        }

        animation_timer.0.tick(time.delta());
        if animation_timer.0.just_finished() {

                // If the animation has reached the last index in the spritesheet stop animating
                let new_sprite_index = death_sprite.index + 1;
                if new_sprite_index == art::DEATH_SPRITE_SHEET_COLUMNS * art::DEATH_SPRITE_SHEET_ROWS {
                    commands.entity(death_entity).despawn();
                    animation_timer.0.paused();
                } else {
                    death_sprite.index = new_sprite_index;
                }
            
        }
    }
    
}