use crate::prelude::{ui::*, *};
use crate::PreviousMenuState;

pub mod components;
pub mod menu;
pub mod pause;
pub mod settings;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            button_processing.run_if(not(in_state(MenuStates::Disable))),
        );
    }
}

// TODO: унифицировать функцию для использования во всех системах без копирования
fn button_processing(
    mut previous_state: ResMut<PreviousMenuState>,
    mut app_exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameStates>>,
    game_state: Res<State<MenuStates>>,
    mut next_state_menu: ResMut<NextState<MenuStates>>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            &ButtonLabel,
            // Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (_, interaction, mut color, button_colors, button_label) in &mut interaction_query {
        match *interaction {
            // TODO: send event for change states
            Interaction::Pressed => match button_label {
                ButtonLabel::StartGame => {
                    next_state_menu.set(MenuStates::Disable);
                    next_state.set(GameStates::Playing);
                }
                ButtonLabel::Continue => {
                    next_state_menu.set(MenuStates::Disable);
                }
                ButtonLabel::Settings => {
                    let current_game_state = game_state.get();
                    previous_state.0 = current_game_state.clone();
                    next_state_menu.set(MenuStates::Setting);
                    info!("setting pressed");
                }
                ButtonLabel::Back => {
                    next_state_menu.set(previous_state.0.clone());
                }
                ButtonLabel::Quit => {
                    app_exit.write(AppExit::Success);
                }
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
