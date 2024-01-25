use bevy::prelude::*;
use crate::{art, generic, AppState, game};
use game::sprite_scaler;

pub const LEVEL_TILE_WIDTH: usize = 4; // How many tiles wide to make the level

#[derive(Resource)]
pub struct BackgroundWallRows { // Contains the total number of background wall rows spawned
    pub rows: u32,
}

impl BackgroundWallRows {
    pub fn new() -> Self {
        BackgroundWallRows{rows: 0}
    }
}

#[derive(Component)]
pub struct BackgroundWall {
    row: u32,
    y_coordinate: f32,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_background_walls)
            .add_systems(Update, (spawn_background_walls, despawn_walls).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)));
    }
}

// Spawn a row of background walls when needed, and despawn unnecasary rows
pub fn spawn_background_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    screen_information: Res<generic::ScreenInformation>,
    bg_walls_query: Query<&BackgroundWall>,
    mut bg_walls_rows: ResMut<BackgroundWallRows>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
) {
    let wall_world_size = Vec2::new(
        art::WALL_SPRITE_SIZE.x * scale_factor.current,
        art::WALL_SPRITE_SIZE.y * scale_factor.current,
    );

    // Get y coordinate of the last wall row
    let mut last_y_coordinate: Option<f32> = None;
    for wall in bg_walls_query.iter() {
        //println!("{}", wall.y_coordinate);
        if wall.row == bg_walls_rows.rows - 1 {
            last_y_coordinate = Some(wall.y_coordinate);
        }   
    }
    let last_y_coordinate = last_y_coordinate;

    // Determine number of background rows to spawn, and the initial spawn y coordinate
    let spawn_rows: usize;
    let init_spawn_y_coordinate: f32;
    if let Some(last_y_coordinate) = last_y_coordinate {
        if last_y_coordinate > screen_information.y_visible_area.min - (wall_world_size.y * 2.0) { // Only spawn new background walls when the old ones are close to the bottom of the screen
            spawn_rows = 1;
            init_spawn_y_coordinate = last_y_coordinate - wall_world_size.y;
        } else {
            spawn_rows = 0;
            init_spawn_y_coordinate = 0.0;
        }
        
    } else { // When there are no background walls the screen is filled
        spawn_rows = (screen_information.window_height as usize / wall_world_size.y as usize) + 5;

        // Spawn the first wall at the top of the screen
        init_spawn_y_coordinate = screen_information.window_height - (wall_world_size.y / 2.0);
    }

    // Spawn rows
    for row in 0..spawn_rows {
        let spawn_y_coordinate = init_spawn_y_coordinate - (wall_world_size.y * row as f32);

        for i in 1..LEVEL_TILE_WIDTH + 1 {
            let x_pos = (i as f32 * wall_world_size.x) - (wall_world_size.x / 2.0) + screen_information.x_deadspace;
            commands.spawn(
                (
                    game::GameComponent,
                    BackgroundWall {
                        row:  bg_walls_rows.rows,
                        y_coordinate: spawn_y_coordinate
                    },
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(x_pos as f32, spawn_y_coordinate, 0.0),
                            scale: Vec3::splat(scale_factor.current),
                            ..default()
                        },
                        texture: asset_server.load(art::WALL_SPRITE_PATH),
                        ..default()
                    }
                )
            );
        }

        bg_walls_rows.rows += 1;
    }
}

// Scroll walls to make the illusion of the player moving downwards
// The scroll speed is the players y speed
fn despawn_walls(
    mut commands: Commands,
    mut bg_walls_query: Query<(Entity, &BackgroundWall)>,
    screen_information: Res<generic::ScreenInformation>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
) {
    let wall_max_y = screen_information.y_visible_area.max + (art::WALL_SPRITE_SIZE.y * scale_factor.current);

    for (entity, bg_wall) in bg_walls_query.iter_mut() {
        if bg_wall.y_coordinate > wall_max_y {
            commands.entity(entity).despawn(); // Despawn walls which have gone out of the visible area
        }
    }
}