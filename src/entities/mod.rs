pub mod components;
pub mod events;
pub mod game_states;
pub mod resources;

use crate::prelude::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameStates>()
            .enable_state_scoped_entities::<GameStates>()
            .init_state::<MenuStates>()
            .enable_state_scoped_entities::<MenuStates>()
            .init_state::<SettingsStates>()
            .enable_state_scoped_entities::<SettingsStates>()
            .insert_resource(PreviousMenuState(MenuStates::default()))
            .insert_resource(GlobalVolume(0.3))
            .insert_resource(MovementIntent(Vec2::ZERO));
    }
}