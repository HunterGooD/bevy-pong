use crate::prelude::*;

#[derive(Resource)]
pub struct PreviousMenuState(pub MenuStates);

#[derive(Resource)]
pub struct GlobalVolume(pub f64);

#[derive(Resource)]
pub struct MovementIntent(pub Vec2);
