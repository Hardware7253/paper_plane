use bevy::prelude::*;

// Player sprite sheet information
pub const PLAYER_SPRITE_SHEET_PATH: &'static str = "sprites/PaperPlanes.png";
pub const PLAYER_SPRITE_SHEET_ROWS: usize = 1;
pub const PLAYER_SPRITE_SHEET_COLUMNS: usize = 6;
pub const PLAYER_SPRITESHEET_INDICES: usize = PLAYER_SPRITE_SHEET_ROWS * PLAYER_SPRITE_SHEET_COLUMNS;
pub const PLAYER_SPRITE_SHEET_START_INDEX: usize = 0; // Sprite sheet index which the player spawns with
pub const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(32.0, 32.0);

// Player death animation sprite sheet information
// Plays upon game over at the location which the player died
pub const DEATH_SPRITE_SHEET_PATH: &'static str = "sprites/PlaneExplode.png";
pub const DEATH_SPRITE_SHEET_ROWS: usize = 1;
pub const DEATH_SPRITE_SHEET_COLUMNS: usize = 8;
pub const DEATH_SPRITE_SIZE: Vec2 = Vec2::new(32.0, 32.0);

// Wall sprite infromation
pub const WALL_SPRITE_PATH: &'static str = "sprites/Wall.png";
pub const WALL_SPRITE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

// Platform sprite information
pub const PLATFORM_CORNER_SPRITE_PATH: &'static str = "sprites/PlatformCorner.png";
pub const PLATFORM_STRAIGHT_SPRITE_PATH: &'static str = "sprites/PlatformStraight.png";
pub const PLATFORM_SPRITE_SIZE: Vec2 = Vec2::new(8.0, 8.0);

// Camera background color
pub const CAMERA_BACKGROUND_SRPITE: &'static str = "sprites/Background.png";
pub const CAMERA_BACKGROUND_HEX: &'static str = "1a1c2c";