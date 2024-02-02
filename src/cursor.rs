use bevy::prelude::*;
use bevy::window;
use std::time::Instant;

#[derive(Resource)]
struct LastCursorMovement(Instant);

const HIDE_SECONDS: f32 = 2.0; // Time the cursor has to spend idle before dissapearing

pub struct AutoHideCursorPlugin;

impl Plugin for AutoHideCursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LastCursorMovement(Instant::now()))
            .add_systems(Update, auto_hide_cursor);
    }
}

fn auto_hide_cursor(
    mut window_query: Query<&mut Window>,
    mut cursor_moved: EventReader<window::CursorMoved>,
    mut last_cursor_movement: ResMut<LastCursorMovement>,
) {
    for _ in cursor_moved.read() {
        last_cursor_movement.0 = Instant::now();
    }

    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.visible = last_cursor_movement.0.elapsed().as_secs_f32() < HIDE_SECONDS;
    }
}