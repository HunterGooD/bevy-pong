use crate::prelude::{ui::*, *};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::MainMenu), setup_menu)
            .add_systems(OnExit(MenuStates::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

// TODO: унифицировать создание меню и кнопок для него
fn setup_menu(mut commands: Commands) {
    info!("menu");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                ..default()
            },
            GlobalZIndex(11),
            Menu,
        ))
        .with_child((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            children![
                default_button("Play", ButtonLabel::StartGame),
                default_button("Settings", ButtonLabel::Settings),
                default_button("Quit", ButtonLabel::Quit),
            ],
        ));
}

// #[derive(Component)]
// struct ChangeState(GameState);

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
