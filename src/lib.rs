#![allow(clippy::type_complexity)]
mod audio;
mod entities;
mod event_managers;
mod game_plugins;
mod input;
mod loading;
mod prelude;
mod save_manager;
mod ui;
mod utils;

use crate::prelude::{ui::*, *};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EntityPlugin,
            LoadingPlugin,
            SettingSaveManagerPlugin,
            GameSaveManagerPlugin,
            UIPlugin,
            InGamePlugins,
            InputPlugin,
            InternalAudioPlugin,
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
