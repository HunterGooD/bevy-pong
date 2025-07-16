use crate::{menu::ButtonColors, PlayingStates};
use bevy::{color::palettes::css::DARK_CYAN, prelude::*};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayingStates::Pause), setup_pause)
            .add_systems(OnEnter(PlayingStates::Play), cleanup_pause);
    }
}

#[derive(Component)]
struct Pause;

fn setup_pause(mut commands: Commands) {
    commands
        // main node pause
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::hsla(136., 0.54, 0.43, 0.4)),
            Pause,
        ))
        .with_children(|parent| {
            // pause rectangle
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(60.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    BackgroundColor(Color::hsla(136., 0.54, 0.43, 0.6)),
                    Outline {
                        width: Val::Px(2.),
                        color: DARK_CYAN.into(),
                        offset: Val::Px(0.),
                    },
                ))
                .with_children(|children| {
                    // text pause
                    children
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                        )).with_child((
                        Text::new("Pause menu"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                        ));
                    // Button continue ant etc.
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
        });
}

fn cleanup_pause(mut commands: Commands, menu: Query<Entity, With<Pause>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
