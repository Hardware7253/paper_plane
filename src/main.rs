use bevy::prelude::*;

pub mod game;
pub mod art;
pub mod generic;


pub mod ui;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
   
    #[default]
    MainMenu,
    ResetGame,
    Game,
    GameOver,
}


fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Change ImagePlugin to render sprites with nearest scaling
        .add_plugins((game::GamePlugin, generic::GenericPlugin, ui::UiPlugin))
        .run();
}