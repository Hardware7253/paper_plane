use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{art, game};
use rand::Rng;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..2) {
            0 => return Direction::Left,
            _ => return Direction::Right,
        }
    }

    pub fn reverse(&self) -> Self {
        let reverse = match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        reverse
    }
}

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScreenInformation::new())
            .add_systems(PreStartup, update_screen_information)
            .add_systems(Update, update_screen_information);
    }
}


#[derive(Resource)]
pub struct ScreenInformation {
    pub x_deadspace: f32,
    pub y_visible_area: Range<f32>,
    pub window_width: f32,
    pub window_height: f32,
}

impl ScreenInformation {
    pub fn new() -> Self {
        ScreenInformation {
            x_deadspace: 0.0,
            y_visible_area: Range {min: 0.0, max: 0.0},
            window_width: 0.0,
            window_height: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

pub fn update_screen_information(
    mut screen_information: ResMut<ScreenInformation>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let window = window_query.get_single().unwrap();
    screen_information.window_width = window.width();
    screen_information.window_height = window.height();
    

    screen_information.x_deadspace = calculate_screen_deadspace(window.width());

    // Calculate y visible area if the camera exists, otherwise use the window height
    if let Ok(camera_transform) = camera_query.get_single() {
        screen_information.y_visible_area = calculate_visible_y_area(window.height(), camera_transform.translation.y);   
    } else {
        screen_information.y_visible_area = Range {min: 0.0, max: window.height()}
    }
    
}

// Calculates the maximum and minimum y coordinate on the screen
fn calculate_visible_y_area(window_height: f32, camera_y: f32) -> Range<f32> {
    let half_window_height = window_height / 2.0;

    Range {
        min: camera_y - half_window_height,
        max: camera_y + half_window_height,
    }
}

// Calculates the distance between the background walls and the edge of the screen
fn calculate_screen_deadspace(window_width: f32) -> f32 {
    (window_width - (game::background::LEVEL_TILE_WIDTH as f32 * art::WALL_WORLD_SIZE.x)) / 2.0
}