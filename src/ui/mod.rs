use bevy::prelude::*;
use crate::AppState;
use crate::game::GameState;

pub mod pause_menu;
pub mod main_menu;
pub mod hud;
pub mod game_over_menu;

pub mod helpers;
pub mod styles;

#[derive(Component)]
pub struct GenericButton;

#[derive(Component)]
pub struct GenericBackButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((main_menu::MainMenuPlugin, hud::HudPlugin, pause_menu::PauseMenuPlugin, game_over_menu::GameOverMenuPlugin))
        .add_systems(Update, (button_interactions, back_button_interactions));
    }
}

// Changes button colors when the user interacts with it
pub fn button_interactions(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => *background_color = Color::hex(styles::BUTTON_PRESSED_HEX).unwrap().into(),
            Interaction::Hovered => *background_color = Color::hex(styles::BUTTON_HOVER_HEX).unwrap().into(),
            Interaction::None => *background_color = Color::hex(styles::BUTTON_DEFAULT_HEX).unwrap().into(),
        }
    }
}

// Go back the main menu
pub fn back_button_interactions(
    button_query: Query<&Interaction, (Changed<Interaction>, With<GenericBackButton>)>,
    mut next_gamestate: ResMut<NextState<GameState>>,
    mut next_appstate: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match interaction {
            Interaction::Pressed => {next_appstate.set(AppState::MainMenu); next_gamestate.set(GameState::Running);},
            _ => (), 
        }
    }
}