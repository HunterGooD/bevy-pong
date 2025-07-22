pub(crate) use bevy::prelude::*;
pub(crate) use leafwing_input_manager::prelude::*;

// pub (crate) mod colors {
//     // pub (crate) use bevy::color::*;
//     // pub (crate) use bevy::color::palettes::css::*;
// }
#[cfg(debug_assertions)]
pub(crate) use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(debug_assertions)]
pub(crate) use bevy_inspector_egui::bevy_egui::EguiPlugin;
// TODO: unused
// #[cfg(debug_assertions)]
// pub(crate) use bevy_inspector_egui::prelude::*;
#[cfg(debug_assertions)]
pub(crate) use bevy_inspector_egui::quick::WorldInspectorPlugin;

// LOCAL IMPORTS
pub(crate) use crate::entities::components::*;
/**
    maybe use this structure for detailed imports

    pub(crate) mod resources {
        pub(crate) use crate::entities::resources::*;
    }
    pub(crate) mod components {
        pub(crate) use crate::entities::components::*;
    }
    pub(crate) mod state {
        pub(crate) use crate::entities::game_states::*;
    }
**/
pub(crate) use crate::entities::game_states::*;
pub(crate) use crate::entities::resources::*;
pub(crate) use crate::entities::*;
pub(crate) mod ui {
    pub(crate) use crate::ui::components::button::*;
    pub(crate) use crate::ui::components::label::*;
    pub(crate) use crate::ui::components::style::*;
    pub(crate) use crate::ui::*;
}
pub(crate) use crate::audio::*;
pub(crate) use crate::game_plugins::*;
pub(crate) use crate::input::*;
pub(crate) use crate::loading::*;
// pub(crate) use crate::event_managers::state_manager::*;
