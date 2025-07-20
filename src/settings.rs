use crate::menu::{ButtonColors, ButtonLabel};
use crate::{MenuStates, PreviousMenuState};
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Setting), setup_settings)
            .add_systems(OnExit(MenuStates::Setting), cleanup_settings)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(MenuStates::Setting)),
            );
    }
}

#[derive(Component)]
struct Setting;

// TODO: унифицировать создание меню и кнопок для него
fn setup_settings(mut commands: Commands) {
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
            Setting,
        ))
        .with_children(|children| {
            // text pause
            children
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_child((
                    Text::new("Settings menu"),
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
                    ButtonLabel::Back,
                ))
                .with_child((
                    Text::new("Back"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

// TODO: унифицировать функцию для использования во всех системах без копирования
fn click_play_button(
    previous_state: Res<PreviousMenuState>,
    mut next_state_menu: ResMut<NextState<MenuStates>>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &ButtonLabel,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (_, interaction, mut color, button_colors, button_label) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button_label {
                ButtonLabel::Back => {
                    // TODO: придумать систему которая бы сама контролировала смену состояний через ивенты чтобы изменения были в одном месте и трекать откуда пришел ивент
                    next_state_menu.set(previous_state.0.clone());
                }
                _ => (),
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_settings(mut commands: Commands, menu: Query<Entity, With<Setting>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
