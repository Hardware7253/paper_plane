use bevy::prelude::*;

pub mod game;
pub mod art;
pub mod generic;


pub mod ui;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
   
    #[default]
    MainMenu,

    GameSetup,
    Game,
    GameCleanup,
}

#[derive(Event)]
pub struct GameCleanupEvent {
    next_state: AppState,
}


fn main() {
    App::new()
        .add_event::<GameCleanupEvent>()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Change ImagePlugin to render sprites with nearest scaling
        .add_plugins((game::GamePlugin, generic::GenericPlugin, ui::UiPlugin))

        .add_systems(OnEnter(AppState::GameCleanup), game_cleanup_transition)
        .add_systems(OnEnter(AppState::GameSetup), game_setup_transition)

        .run();
}

// After cleanup transition enter the AppState given by the event
fn game_cleanup_transition(
    mut cleanup_event: EventReader<GameCleanupEvent>,
    mut next_game_state: ResMut<NextState<game::GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    next_game_state.set(game::GameState::Running);
    for event in cleanup_event.read() {
        next_app_state.set(event.next_state);
        
    }
}

// After the game is setup transition to AppState::Game
fn game_setup_transition(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}