use bevy::prelude::{ButtonInput, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

impl GameControl {
    pub fn is_pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
            GameControl::Escape => {
                keyboard_input.pressed(KeyCode::Escape)
            }
        }
    }

    pub fn is_just_pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight)
            }
            GameControl::Escape => {
                keyboard_input.just_pressed(KeyCode::Escape)
            }
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<ButtonInput<KeyCode>>) -> f32 {
    if control.is_pressed(input) {
        1.0
    } else {
        0.0
    }
}
