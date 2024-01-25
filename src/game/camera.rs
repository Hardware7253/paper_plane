use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use crate::{art, game, AppState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera.after(game::player::move_player).run_if(in_state(AppState::Game)).run_if(in_state(game::GameState::Running)))

            .add_systems(OnEnter(AppState::GameCleanup), reset_camera_position)
            .add_systems(Update, reset_camera_position.run_if(in_state(AppState::MainMenu)));
    }
}

// Spawn camera
fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            camera_2d: Camera2d {clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::hex(art::CAMERA_BACKGROUND_HEX).unwrap())},
            ..default()
        }
    );
}

// Move camera to players y position
fn move_camera(mut camera_query: Query<&mut Transform, (With<Camera>, Without<game::player::Player>)>, player_query: Query<&Transform, With<game::player::Player>>) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            camera_transform.translation.y = player_transform.translation.y;
        }
    }    
}

// Reset camera position unconditionally when called when in AppState::GameCleanup
// Else reset camera when a resize event occurs
pub fn reset_camera_position(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut resize_event: EventReader<WindowResized>,
    app_state: Res<State<AppState>>,
    
) {
    let window = window_query.get_single().unwrap();
    if let Ok(camera_transform) = &mut camera_query.get_single_mut() {

        let mut resize = match app_state.get() {
            AppState::GameCleanup => true,
            _ => false,
        };

        for _ in resize_event.read() {
            resize = true;
        }
        

        if resize {
            camera_transform.translation.x = window.width() / 2.0;
            camera_transform.translation.y = window.height() / 2.0;
        }
        
    }
}