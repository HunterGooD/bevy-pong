use crate::menu::ButtonLabel;
use crate::{menu::ButtonColors, MenuStates, PreviousMenuState};
use bevy::{color::palettes::css::DARK_CYAN, prelude::*};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::PauseMenu), setup_pause)
            .add_systems(OnExit(MenuStates::PauseMenu), cleanup_pause)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(MenuStates::PauseMenu)),
            );
    }
}

#[derive(Component)]
struct Pause;

// TODO: унифицировать создание меню и кнопок для него
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
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },))
                        .with_child((
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
                            ButtonLabel::Continue,
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
                    let button_colors = ButtonColors::default();
                    children
                        .spawn((
                            Button,
                            ButtonLabel::Settings,
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
                            Text::new("Settings"),
                            TextFont {
                                font_size: 40.0,
                                ..default()
                            },
                            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                        ));
                });
        });
}
// TODO: унифицировать функцию для использования во всех системах без копирования
fn click_play_button(
    mut previous_state: ResMut<PreviousMenuState>,
    mut next_state_menu: ResMut<NextState<MenuStates>>,
    game_state: Res<State<MenuStates>>,
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
                ButtonLabel::Continue => {
                    next_state_menu.set(MenuStates::Disable);
                }
                ButtonLabel::Settings => {
                    // TODO: придумать систему которая бы сама контролировала смену состояний через ивенты чтобы изменения были в одном месте и трекать откуда пришел ивент
                    let current_game_state = game_state.get();
                    previous_state.0 = current_game_state.clone();
                    next_state_menu.set(MenuStates::Setting);
                    info!("setting pressed");
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

fn cleanup_pause(mut commands: Commands, menu: Query<Entity, With<Pause>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
