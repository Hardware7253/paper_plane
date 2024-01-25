use bevy::prelude::*;
use crate::{art, generic, game, AppState};
use game::sprite_scaler;
use generic::Direction;
use std::f32::consts::PI;

const START_DIRECTION: Direction = Direction::Right; // Direction player starts facing
const DEATH_ANIMATION_FPS: f32 = 8.0;

const AUTO_MOVE_AV: f32 = 6.0; // Radians per second which the player turns when a steering button is held
const ANGLE_RANGE_RAD: generic::Range<f32> = generic::Range {min: PI / -2.0, max: PI / 2.0}; // Miniumum and maximum angle for player

#[derive(Component, Debug)]
pub struct Player {
    pub speed: Vec2, // X and y speed

    facing: Direction, // Direction the player is currently facing
    angle_rad: f32, 
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
            .add_systems(Update, animate_death.run_if(in_state(game::GameState::GameOver)))
            .add_systems(Update, (change_angle, set_player_heading, calculate_speed, move_player).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}

// Spawns player and initializes death animation
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    screen_infromation: Res<generic::ScreenInformation>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
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
    let player_world_width = art::PLAYER_SPRITE_SIZE.x * scale_factor.current;
    let player_spawn_x = match START_DIRECTION {
        Direction::Left => screen_infromation.window_width - screen_infromation.x_deadspace - player_world_width / 2.0, 
        Direction::Right => screen_infromation.x_deadspace + player_world_width / 2.0,
    };

    // Spawn player
    commands.spawn(
        (
            game::GameComponent,
            Player {
                speed: Vec2::new(0.0, 0.0),
                facing: START_DIRECTION.reverse(),

                angle_rad: match START_DIRECTION { // Radians from the center
                    Direction::Left => ANGLE_RANGE_RAD.min,
                    Direction::Right => ANGLE_RANGE_RAD.max,
                },
            },

            SpriteSheetBundle {
                texture_atlas: texture_atlases.add(player_texture_atlas),
                sprite: TextureAtlasSprite::new(art::PLAYER_SPRITE_SHEET_START_INDEX),
                transform: Transform {
                    translation: Vec3::new(player_spawn_x, screen_infromation.window_height / 2.0, 2.0),
                    scale: Vec3::splat(scale_factor.current),
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
                    scale: Vec3::splat(scale_factor.current),
                    ..default()
                },
                ..default()
            },
        )
    );
}

// Changes the players angle with the steering keys
fn change_angle(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &TextureAtlasSprite)>,
    time: Res<Time>,
) {
    if let Ok((player, sprite)) = &mut player_query.get_single_mut() {

        let rad_per_sprite = ANGLE_RANGE_RAD.max / art::PLAYER_SPRITESHEET_INDICES as f32;
        let rad_from_sprite_index = generic::reverse_index(sprite.index, art::PLAYER_SPRITESHEET_INDICES) as f32 * rad_per_sprite * player.facing.to_x();

        // Immediately jump to the next angle_rad which corresponds to a spritesheet index
        // This is done to give immideate feedback to the player
        if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right) {
            player.angle_rad = rad_from_sprite_index + rad_per_sprite;
        }
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) {
            player.angle_rad = rad_from_sprite_index - rad_per_sprite;
        }
        

        // Change player angle_rad in the direction of the steering keypress
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            player.angle_rad += AUTO_MOVE_AV / 1.0 * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            player.angle_rad -= AUTO_MOVE_AV / 1.0 * time.delta_seconds();
        }

        // Do not let the angle exceed limits provided by ANGLE_RANGE_RAD
        if player.angle_rad < ANGLE_RANGE_RAD.min {
            player.angle_rad = ANGLE_RANGE_RAD.min;
        }
        if player.angle_rad > ANGLE_RANGE_RAD.max {
            player.angle_rad = ANGLE_RANGE_RAD.max; 
        }
    }
}

// Set players heading based on the players current rotation angle
fn set_player_heading(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), Changed<Player>>,
) {
    if let Ok((player, sprite)) = &mut player_query.get_single_mut() {
        let angle = player.angle_rad.abs();
        let sprite_sheet_index = generic::map(angle, ANGLE_RANGE_RAD.truncate(), generic::Range {min: 0.0, max: art::PLAYER_SPRITESHEET_INDICES as f32 - 1.0});
        
        // Reverse sprite sheet index because it decreases as angle increases
        let reverse_index = (sprite_sheet_index as usize as i32 - (art::PLAYER_SPRITESHEET_INDICES - 1) as i32).abs() as usize;
        sprite.index = reverse_index;

        // Flip srite once it is left of the center line
        // Don't flip sprite when it is in the center
        // Additionally set the direction which the player is facing
        //
        // Because the center sprite is essentially shown twice it is on screen twice as long as the others
        // This behaviour is expected and wanted
        // It makes it easier for the player to stop in the center position
        if sprite_sheet_index as usize != 0 {
            if player.angle_rad < 0.0 {
                sprite.flip_x = true;
                player.facing = Direction::Left;
            } else {
                sprite.flip_x = false;
                player.facing = Direction::Right;
            }
        }
    }
}

// Calculate speed based on player spritesheet index
fn calculate_speed(mut player_query: Query<(&mut Player, &TextureAtlasSprite)>, game: Res<game::Game>) {
    if let Ok((player, sprite)) = &mut player_query.get_single_mut() {

        // Calculate how much the x and y speed changes each time the player spritesheet index is incremented / deincremented
        let speed_steps = Vec2::new(
            game.difficulty.player_max_speed.x / (art::PLAYER_SPRITESHEET_INDICES - 1) as f32,
            game.difficulty.player_max_speed.y / art::PLAYER_SPRITESHEET_INDICES as f32
        );

        // Calculate speed
        player.speed = Vec2::new(
            speed_steps.x * generic::reverse_index(sprite.index, art::PLAYER_SPRITESHEET_INDICES) as f32, // As the sprite index increases the x speed should decrease (because at index 0 the plane is facing straight down)
            speed_steps.y * (sprite.index + 1) as f32
        )
    }
}

// Moves players x coordinate every frame according to player speed
pub fn move_player(mut player_query: Query<(&mut Transform, &Player)>, time: Res<Time>) {
    if let Ok((mut transform, player)) = player_query.get_single_mut() {
    
        transform.translation.x += player.speed.x * player.facing.to_x() * time.delta_seconds();
        transform.translation.y -= player.speed.y * time.delta_seconds();
    }
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