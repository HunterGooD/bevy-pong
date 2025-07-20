#![allow(clippy::type_complexity)]
// TODO: добавить окно настроек придумать как оптимизировать это дело чтобы в настройки зайти и из паузы
// TODO: изменение звука и т.п.
mod actions;
mod audio;
mod loading;
mod menu;
mod pause;
mod player;
mod settings;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::pause::PauseMenuPlugin;
use crate::player::PlayerPlugin;
use crate::settings::SettingsPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
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
#[derive(Resource)]
pub struct PreviousMenuState(pub MenuStates);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<MenuStates>()
            .insert_resource(PreviousMenuState(MenuStates::default()))
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                SettingsPlugin,
                ActionsPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                PauseMenuPlugin,
            ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
            ));
        }
    }
}
