use bevy::prelude::*;
use std::f32::consts::PI;
use crate::{art, generic, AppState, game};
use game::sprite_scaler;
use rand::Rng;
use generic::Direction;

const DOUBLE_SIDED_PLATFORM_CHANCE: usize = 10; // % Change for a double sided platform to spawn for every platform spawn

// For spawning the first platform
const FIRST_PLATFORM_SIDE: Direction = Direction::Left;
const FIRST_PLATFORM_Y: f32 = 1.0 / 4.0; // How far up the screen the first platform should spawn


#[derive(Resource)]
pub struct Platforms { 
    pub total_platforms: u32, // The total number of platforms spawned
    pub platforms_vec: Vec<Platform>, // Contains every existing platform
} 

#[derive(Component, Copy, Clone)]
pub struct Platform {
    pub index: u32,
    pub hitbox: [generic::Range<f32>; 2], // x and y hitbox
    pub dimensions: [i32; 2], // x and y platform dimensions
    pub side: generic::Direction, // Side of the screen the platform is spawned from
}

impl Platforms {
    pub fn new() -> Self {
        Platforms {total_platforms: 0, platforms_vec: Vec::new()}
    }
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_platforms)
            .add_systems(Update, (spawn_platforms, despawn_platforms.after(spawn_platforms)).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}

// Responsible for spawning new platforms when needed
// Platform dimensions and location are specified by game difficulty parameters
fn spawn_platforms(
    screen_information: Res<generic::ScreenInformation>,
    mut platforms: ResMut<Platforms>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<game::Game>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
) {
    let difficulty = &game.difficulty;
    let mut rng = rand::thread_rng();

    // Calculate long a platform would need to be to cross the entire screen
    let platform_world_width = art::PLATFORM_SPRITE_SIZE.x * scale_factor.current;
    let platforms_across_screen: i32 = ((screen_information.window_width - (screen_information.x_deadspace * 2.0)) / platform_world_width) as i32;

    // Continually add platforms untill they fill slightly beyond the visible area
    let mut sufficient_platforms = false;
    while !sufficient_platforms {
        let platform_gap = rng.gen_range(difficulty.platform_gap.min..difficulty.platform_gap.max); // Generate random platform gap
        
        // Get last platform if it exists
        // Otherwise create an imaginary last platform, whose data will be used to spawn the first platform in the correct position
        let last_platform: Platform;
        if platforms.platforms_vec.len() > 0 {
            last_platform = platforms.platforms_vec[platforms.platforms_vec.len() - 1];
        } else {
            last_platform = Platform {
                index: 0,
                hitbox: [
                    generic::Range {min: 0.0, max: 0.0}, // X
                    generic::Range {min: (screen_information.window_height * FIRST_PLATFORM_Y) + platform_gap, max: 0.0}, // Y
                ],
                dimensions: [0, 0],
                side: FIRST_PLATFORM_SIDE.reverse(),
            };
        }

        let last_platform_y_min = last_platform.hitbox[1].min;

        // How many platform tiles length each platform can vary from the last
        let platform_sprites_per_player_sprite: i32 = ((art::PLAYER_SPRITE_SIZE.x * scale_factor.current) / platform_world_width) as i32;
        let platform_length_variance: generic::Range<i32> = generic::Range {
            min: platform_sprites_per_player_sprite * -1,
            max: platform_sprites_per_player_sprite * 1,
        };

        // Only spawn a new platform when the last platform is close to the visible area
        if last_platform_y_min > screen_information.y_visible_area.min - (platform_gap * 4.0) {

            // Calculate new platform dimensions
            let platform_dimensions = [
                platforms_across_screen / 2 + rng.gen_range(platform_length_variance.min..platform_length_variance.max),
                difficulty.platform_height,
            ];

            // This platform will spawn on the opposite screen side to the previous platform
            let platform_side = last_platform.side.reverse();

            // Create an uncomplete platform hitbox
            // The only information which needs to be known right now is the top of the platform
            // The rest can be calculated afterwards
            let platform_hitbox = [
                generic::Range {min: 0.0, max: 0.0}, // X
                generic::Range {min: 0.0, max: last_platform_y_min - platform_gap}, // Y
            ];
            
            let platform = Platform {
                index: platforms.total_platforms,
                hitbox: platform_hitbox,
                dimensions: platform_dimensions,
                side: platform_side,
            };

            // Spawn 2 platforms DOUBLE_SIDED_PLATFORM_CHANCE % of times
            let spawn_platforms = if rng.gen_range(1..100) < DOUBLE_SIDED_PLATFORM_CHANCE {
                2
            } else {
                1
            };

            for i in 0..spawn_platforms {

                // When a second platform is spawned it goes on the opposite side of the screen
                // It's length is set such that there is a small gap between the platforms for the player to pass through
                let current_platform: Platform;
                if i > 0 {
                    current_platform = Platform {
                        index: platforms.total_platforms,
                        hitbox: platform_hitbox,
                        dimensions: [
                            platforms_across_screen - platform_dimensions[0] - (platform_sprites_per_player_sprite * 2),
                            platform_dimensions[1],
                        ],
                        side: platform_side.reverse(),
                    }
                } else {
                    current_platform = platform;
                }

                draw_platform(current_platform, &mut commands, &asset_server, &mut platforms, &screen_information, scale_factor.current)
            }
            

        } else {

            // Once the platforms fill slightly beyond the visible area no more need to be spawned
            sufficient_platforms = true;
            continue;
        }
    }

    

        
}

// Draws a platform on either the left or right side of the level with a specified size
fn draw_platform(
    mut platform: Platform,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    platforms: &mut ResMut<Platforms>,
    screen_information: &generic::ScreenInformation,
    scale_factor: f32
) {
    let platform_world_size = Vec2::new(
        art::PLATFORM_SPRITE_SIZE.x * scale_factor,
        art::PLATFORM_SPRITE_SIZE.y * scale_factor,
    );

    // Determine location of the first sprite, and set tile rotations which change depending on which side of the screen the platform is spawned on
    let sprite_spawn_x: f32;
    let straight_tile_rotations: [f32; 3];
    let corner_tile_rotations: [f32; 3];
    let x_direction: f32;
    
    if platform.side == Direction::Left {
        sprite_spawn_x = screen_information.x_deadspace +  (platform_world_size.x / 2.0);
        straight_tile_rotations = [0.0, 1.5, 1.0];
        corner_tile_rotations = [1.5, 1.0, 0.0];
        x_direction = 1.0; // Platform grows into positive x direction
    } else {
        sprite_spawn_x = screen_information.window_width - screen_information.x_deadspace -  (platform_world_size.x / 2.0);
        straight_tile_rotations = [0.0, 0.5, 1.0];
        corner_tile_rotations = [0.0, 0.5, 0.0];
        x_direction = -1.0; // Platform grows into negative x direction
    }

    let tile_rotations = [straight_tile_rotations, corner_tile_rotations];
    let first_sprite_location = Vec2::new(sprite_spawn_x, platform.hitbox[1].max - (platform_world_size.y / 2.0));

    // Calculate the rest of the platform hitbox (allready given y max from the parent function)
    platform.hitbox[0] = generic::Range { // X range
        max: first_sprite_location.x + (platform.dimensions[0] as f32 * platform_world_size.x * x_direction),
        min: first_sprite_location.x,
    };
    platform.hitbox[1].min = platform.hitbox[1].max - (platform_world_size.y * platform.dimensions[1] as f32); // Y min

    // Spawn all tile pieces of the platform
    for x in 0..platform.dimensions[0] {
        for y in 0..platform.dimensions[1] {

            // Skip coordinates where sprites should not be spawned
            let y_middle = y > 0 && y < platform.dimensions[1] - 1; // True if the current y coordinate is in the middle of the platform (where no sprite should be)
            if x < platform.dimensions[0] - 1 && y_middle {
                continue;
            }

            // Where the sprite should be spawned
            let sprite_location = Vec2::new(first_sprite_location.x + (x_direction * platform_world_size.x * x as f32), first_sprite_location.y - (platform_world_size.y * y as f32));
            
            // Set the sprite to a corner or straight piece based on the x and y coordinate
            let sprite_path: &str;
            let tile_index: usize;
            if x == platform.dimensions[0] - 1 && (y == 0 || y == platform.dimensions[1] - 1) { // Detect corner piece
                sprite_path = art::PLATFORM_CORNER_SPRITE_PATH;
                tile_index = 1;
            } else { // Detect straight piece
                sprite_path = art::PLATFORM_STRAIGHT_SPRITE_PATH;
                tile_index = 0;
            }

            // Determine rotation index from x and y coordinates
            let rotation_index: usize;
            if x < platform.dimensions[0] - 1 {
                if y == platform.dimensions[1] - 1 {
                    rotation_index = 2;
                } else {
                    rotation_index = 0;
                }
            } else {
                if y_middle {
                    rotation_index = 1;
                } else {
                    if y == 0 {
                        rotation_index = 0;
                    } else {
                        rotation_index = 1;
                    }
                }
            }

            // Spawn sprite
            commands.spawn(
                (
                    game::GameComponent,
                    platform,
                    SpriteBundle {
                        texture: asset_server.load(sprite_path),
                        transform: Transform {
                            translation: Vec3::new(sprite_location.x, sprite_location.y, 1.0),
                            rotation: Quat::from_rotation_z(PI * tile_rotations[tile_index][rotation_index]),
                            scale: Vec3::splat(scale_factor),
                        },
                        ..default()
                    },
                )
            );
        }
    }

    // Update platforms vec with new platform
    platforms.platforms_vec.push(platform);
    platforms.total_platforms += 1;
}


// Despawn platforms which have gone out of the visible area
// Also increase the score when a platform is despawned
fn despawn_platforms(
    mut commands: Commands,
    platform_query: Query<(Entity, &Platform)>,
    mut platforms: ResMut<Platforms>,
    mut game: ResMut<game::Game>,
    screen_information: Res<generic::ScreenInformation>,
    mut score_increase: EventWriter<game::ScoreIncrease>
) {
    let mut removed_indices: Vec<u32> = Vec::new(); // Platform indices removed from the platforms vec

    for (entity, platform) in platform_query.iter() {
        if platform.hitbox[1].min > screen_information.y_visible_area.max {
            commands.entity(entity).despawn();

            // Only remove this platform from the platforms vec once (because there is only one entry per platform)
            if !removed_indices.contains(&platform.index) {
                game.score += 1;
                score_increase.send(game::ScoreIncrease);

                removed_indices.push(platform.index);
                platforms.platforms_vec.remove(0);
            }   
        }
    }
}