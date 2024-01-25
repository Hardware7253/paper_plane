use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{art, game};
use game::sprite_scaler;
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
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn to_x(&self) -> f32 {
        match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl Range<f32> {
    pub fn new() -> Self {
        Range {min: 0.0, max: 0.0}
    }

    // Set min value to 0 when it's negative
    pub fn truncate(self) -> Self {
        if self.min < 0.0 {
            return Range {
                min: 0.0,
                max: self.max,
            }
        }
        self
    }

    // Adjusts both values of a range so that the min value starts at 0
    // E.g. Range {min: -1.0, max: 1.0} -> Range {min: 0.0, max: 2.0}
    pub fn zero(self) -> Self {
        if self.min == 0.0 {
            return self;
        }

        // Determine direction to move the max value
        let direction: f32;
        if self.min < 0.0 {
            direction = 1.0;
        } else {
            direction = -1.0;
        }

        Range {
            min: 0.0,
            max: self.max + self.min.abs() * direction
        }
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

pub struct GenericPlugin;

impl Plugin for GenericPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScreenInformation::new())

            .add_systems(PreStartup, update_screen_information)
            .add_systems(Update, update_screen_information);
    }
}

// Updates screen information
// Used by game systems
pub fn update_screen_information(
    mut screen_information: ResMut<ScreenInformation>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, With<Camera>>,
    scale_factor: Res<sprite_scaler::ScaleFactor>,
) {
    if let Ok(window) = window_query.get_single() {
        screen_information.window_width = window.width();
        screen_information.window_height = window.height();
        
        let wall_world_width = art::WALL_SPRITE_SIZE.x * scale_factor.current;
        screen_information.x_deadspace = calculate_screen_deadspace(window.width(), wall_world_width);

        // Calculate y visible area if the camera exists, otherwise use the window height
        if let Ok(camera_transform) = camera_query.get_single() {
            screen_information.y_visible_area = calculate_visible_y_area(window.height(), camera_transform.translation.y);   
        } else {
            screen_information.y_visible_area = Range {min: 0.0, max: window.height()}
        }
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
fn calculate_screen_deadspace(window_width: f32, wall_world_width: f32) -> f32 {
    (window_width - (game::background::LEVEL_TILE_WIDTH as f32 * wall_world_width)) / 2.0
}

// Map an input that can be within input_range to an output that can be within output_range
pub fn map(input: f32, input_range: Range<f32>, output_range: Range<f32>) -> f32 {

    // Get absolute value of the range fields
    let input_range_zeroed = input_range.zero();
    let output_range_zeroed = output_range.zero();

    // Adjust the input to match the absolute values of the input_range
    let input_adjusted = if input_range.min < 0.0 {
        input + input_range.min.abs()
    } else if input_range.min > 0.0 {
        input - input_range.min
    } else {
        input
    };

    let ratio = input_adjusted / input_range_zeroed.max.abs();
    let output_unadjusted = output_range_zeroed.max * ratio;
    
    // Switch back from zeroed output_range for final result
    if output_range.min < 0.0 {
        output_unadjusted + output_range.min
    } else if output_range.min > 0.0 {
        output_unadjusted - output_range.min
    } else {
        output_unadjusted
    }
}

pub fn reverse_index(index: usize, indices: usize) -> usize {
    (index as f32 - (indices - 1) as f32).abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input_range = Range {
            min: 200.0,
            max: 300.0,
        };

        let output_range = Range {
            min: -1.0,
            max: 1.0,
        };

        assert_eq!(map(250.0, input_range, output_range), 0.0);
    }

    #[test]
    fn test_reverse_index() {
        assert_eq!(reverse_index(0, 16), 15);
    }

    #[test]
    fn test_range_zero_neg() {
        let before = Range {min: -1.0, max: 1.0}.zero();
        let after = Range {min: 0.0, max: 2.0};

        assert_eq!(before, after);
    }

    #[test]
    fn test_range_zero_pos() {
        let before = Range {min: 10.0, max: 30.0}.zero();
        let after = Range {min: 0.0, max: 20.0};

        assert_eq!(before, after);
    }
}