use crate::prelude::*;

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStates {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // add splash with image and effect fade out
    // Splash,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash, InspectorOptions)]
pub enum MenuStates {
    #[default]
    Disable,
    MainMenu,
    PauseMenu,
    Setting,
    GameOver,
}
