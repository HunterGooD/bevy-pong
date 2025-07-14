use crate::{PlayingStates, menu::ButtonColors};
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlayingStates::Pause), setup_pause)
            .add_systems(OnEnter(PlayingStates::Play), cleanup_pause);
    }
}

#[derive(Component)]
struct Pause;

fn setup_pause(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Pause,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                ))
                .with_child((
                    Text::new("Continue"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn cleanup_pause(mut commands: Commands, menu: Query<Entity, With<Pause>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}