use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

pub struct ScreenModePlugin;

impl Plugin for ScreenModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_screen_mode);
    }
}

fn change_screen_mode(keyboard_input: Res<Input<KeyCode>>, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.get_single_mut().unwrap();

    if keyboard_input.just_pressed(KeyCode::F11) {
        window.mode = cycle_window_mode(window.mode);
    }
}

fn cycle_window_mode(mode: WindowMode) -> WindowMode {
    match mode {
        WindowMode::Windowed => WindowMode::BorderlessFullscreen,
        WindowMode::BorderlessFullscreen => WindowMode::Windowed,
        _ => WindowMode::Windowed,
    }
}