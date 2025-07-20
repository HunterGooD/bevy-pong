use crate::{GameState, MenuStates, PreviousMenuState};
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::MainMenu), setup_menu)
            .add_systems(OnEnter(GameState::Menu), setup_camera)
            .add_systems(
                Update,
                click_play_button.run_if(in_state(MenuStates::MainMenu)),
            )
            .add_systems(OnExit(MenuStates::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
pub struct ButtonColors {
    pub(crate) normal: Color,
    pub(crate) hovered: Color,
}

#[derive(Component, Clone, Copy)]
pub enum ButtonLabel {
    StartGame,
    Settings,
    Quit,
    // Settings
    Back,
    //Pause
    Continue,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

// TODO: унифицировать создание меню и кнопок для него
fn setup_menu(mut commands: Commands) {
    info!("menu");
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
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    ButtonLabel::StartGame,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
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

            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    ButtonLabel::Quit,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Exit"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ));
        });
}

#[derive(Component)]
struct ChangeState(GameState);

// TODO: унифицировать функцию для использования во всех системах без копирования
fn click_play_button(
    mut previous_state: ResMut<PreviousMenuState>,
    mut app_exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<MenuStates>>,
    mut next_state_menu: ResMut<NextState<MenuStates>>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &ButtonLabel,
            Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (_, interaction, mut color, button_colors, button_label, change_state) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => match button_label {
                ButtonLabel::StartGame => {
                    // TODO: придумать систему которая бы сама контролировала смену состояний через ивенты чтобы изменения были в одном месте и трекать откуда пришел ивент
                    if let Some(state) = change_state {
                        next_state_menu.set(MenuStates::Disable);
                        next_state.set(state.0.clone());
                    }
                }
                ButtonLabel::Settings => {
                    let current_game_state = game_state.get();
                    previous_state.0 = current_game_state.clone();
                    next_state_menu.set(MenuStates::Setting);
                    info!("setting pressed");
                }
                ButtonLabel::Quit => {
                    app_exit.write(AppExit::Success);
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

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
