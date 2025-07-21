#![allow(clippy::type_complexity)]
// TODO: добавить окно настроек придумать как оптимизировать это дело чтобы в настройки зайти и из паузы
// TODO: изменение звука и т.п.
mod actions;
mod audio;
mod camera;
mod entities;
mod event_managers;
mod loading;
mod player;
mod prelude;
mod ui;

use crate::prelude::{ui::*, *};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .init_state::<MenuStates>()
            .insert_resource(PreviousMenuState(MenuStates::default()))
            .add_plugins((
                LoadingPlugin,
                CameraPlugin,
                UIPlugin,
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
