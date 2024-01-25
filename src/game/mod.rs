use bevy::prelude::*;
use crate::{art, generic, AppState};

const START_PLATFORM_HEIGHT: i32 = 2; // Platform height at the start of the game
const POINTS_PER_LEVEL: i32 = 20; // The level number increments every POINTS_PER_LEVEL points
const MAX_PLATFORM_HEIGHT: i32 = 8;

pub mod player;
pub mod background;
pub mod camera;
pub mod platforms;
pub mod collisions;
pub mod sprite_scaler;

// Component attached to every entity which is apart of the game
#[derive(Component)]
pub struct GameComponent;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct Game {
    pub score: i32,
    pub difficulty: Difficulty,
}

#[derive(Event)]
pub struct ScoreIncrease;

// Struct containing infromation regarding the games difficulty
pub struct Difficulty {
    pub level: i32, // Other fields are derived from the level, which itself is derived from the score

    // Current difficulty
    pub platform_gap: generic::Range<f32>, // The min and max values that two platforms might spawn from each other on the y axis
    pub platform_height: i32, // Platform height in tiles
    pub player_max_speed: Vec2,

    // Difficulty at the start of the game
    start_platform_gap: generic::Range<f32>,
    start_player_max_speed: Vec2,
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

            .add_state::<GameState>()

            // Add game plugins
            .add_plugins((
                player::PlayerPlugin,
                background::BackgroundPlugin,
                platforms:: PlatformPlugin,
                camera::CameraPlugin,
                collisions::CollisionPlugin,
                sprite_scaler::SpriteScalerPlugin,
            ))

            // Game resources have to be reset so the game can function properly if the user wants to play again
            .add_systems(OnEnter(AppState::GameSetup), insert_game_resouorces)

            .add_event::<ScoreIncrease>()
            .add_systems(Update, recalculate_difficulty.run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))

            .add_systems(OnEnter(AppState::GameCleanup), despawn_game_components);

    }
}

fn insert_game_resouorces(mut commands: Commands, scale_factor: Res<sprite_scaler::ScaleFactor>) {
    let player_world_height: f32 = art::PLAYER_SPRITE_SIZE.y * scale_factor.current;

    // Difficulty Settings at the start of the game
    let start_platform_gap: generic::Range<f32> = generic::Range {min: player_world_height * 2.2, max: player_world_height * 2.7};
    let start_player_max_speed: Vec2 = Vec2::new(125.0 * scale_factor.current, 212.5 * scale_factor.current);

    commands.insert_resource(
        Game {
            score: 0,
            difficulty: Difficulty {
                level: 1,
                platform_gap: start_platform_gap,
                platform_height: START_PLATFORM_HEIGHT,
                player_max_speed: start_player_max_speed,
                start_platform_gap: start_platform_gap,
                start_player_max_speed: start_player_max_speed,
            }
        }
    );


    commands.insert_resource(platforms::Platforms::new());
    commands.insert_resource(background::BackgroundWallRows::new());
}

// Recalculates difficulty variables based on the score
// Only fully runs when the score updates
pub fn recalculate_difficulty(mut game: ResMut<Game>, mut score_increase: EventReader<ScoreIncrease>) {
    for _ in score_increase.read() {

        let score = game.score;
        let difficulty = &mut game.difficulty;
        difficulty.level = (score / POINTS_PER_LEVEL) + 1;
        let level = difficulty.level;

        // Even though the difficulty should ramp over time the platforms move further apart
        // This is to make it so that the player can reasonably navigate between platforms at high speeds
        let platform_gap_multiplier = (level as f32 * 0.05) + 1.0;
        difficulty.platform_gap.min = difficulty.start_platform_gap.min * platform_gap_multiplier;
        difficulty.platform_gap.max = difficulty.start_platform_gap.max * platform_gap_multiplier;

        // Increment platform height every new level untill it maxes out
        difficulty.platform_height = START_PLATFORM_HEIGHT - 1 + level;
        if difficulty.platform_height > MAX_PLATFORM_HEIGHT {
            difficulty.platform_height = MAX_PLATFORM_HEIGHT;
        }
        
        // Increase player speed
        let player_speed_multiplier = (level as f32 * 0.07) + 1.0;
        difficulty.player_max_speed.x = difficulty.start_player_max_speed.x * player_speed_multiplier;
        difficulty.player_max_speed.y = difficulty.start_player_max_speed.y * player_speed_multiplier;
    }
}

// Despawn all game components
fn despawn_game_components(mut commands: Commands, game_components_query: Query<Entity, With<GameComponent>>) {
    for entity in game_components_query.iter() {
        commands.entity(entity).despawn();
    }
}