pub mod test_effect;

use bevy_enoki::prelude::*;
use crate::prelude::*;
use crate::effects::test_effect::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
    app.add_plugins((EnokiPlugin, TestFirstEffect));
    }
}