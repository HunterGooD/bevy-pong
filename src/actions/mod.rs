use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::player::Player;
use crate::{GameState, MenuStates};

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions.
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (
                set_pause,
                set_movement_actions.run_if(in_state(MenuStates::Disable)),
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<MenuStates>>,
    mut next_state: ResMut<NextState<MenuStates>>,
) {
    let escape_press = GameControl::Escape.is_just_pressed(&keyboard_input);

    if !escape_press {
        return;
    }

    match state.get() {
        // TODO: придумать систему которая бы сама контролировала смену состояний через ивенты чтобы изменения были в одном месте и трекать откуда пришел ивент
        MenuStates::Disable => next_state.set(MenuStates::PauseMenu),
        MenuStates::PauseMenu => next_state.set(MenuStates::Disable),
        _ => (), // skip others states
    }
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Result {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position() {
        if let Ok((camera, camera_transform)) = camera.single() {
            if let Ok(touch_position) =
                camera.viewport_to_world_2d(camera_transform, touch_position)
            {
                let diff = touch_position
                    - player
                        .single()
                        .map(|transform| transform.translation.xy())
                        .unwrap_or(touch_position);
                if diff.length() > FOLLOW_EPSILON {
                    player_movement = diff.normalize();
                }
            }
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }

    Ok(())
}
