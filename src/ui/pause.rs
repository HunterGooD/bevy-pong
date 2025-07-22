use crate::prelude::{ui::*, *};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::PauseMenu), setup_pause)
            .add_systems(OnExit(MenuStates::PauseMenu), cleanup_pause)
            .add_systems(OnEnter(MenuStates::Disable), cleanup_background);
    }
}

#[derive(Component)]
struct Pause;

#[derive(Component)]
struct PauseBackground;

// TODO: унифицировать создание меню и кнопок для него
fn setup_pause(mut commands: Commands) {
    info!("pause");
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::End,
            ..default()
        },
        GlobalZIndex(10),
        BackgroundColor(Color::hsla(217., 0.0, 0.29, 0.4)),
        PauseBackground,
    ));
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
            Pause,
        ))
        .with_child((
            Node {
                width: Val::Percent(55.0),
                height: Val::Percent(40.0),
                margin: UiRect::bottom(Val::Percent(5.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            children![
                label("Pause"),
                default_button("Continue", ButtonLabel::Continue),
                default_button("Settings", ButtonLabel::Settings),
                default_button("In Menu", ButtonLabel::ToMainMenu),
            ],
        ));
}

fn cleanup_pause(mut commands: Commands, menu: Query<Entity, With<Pause>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
fn cleanup_background(mut commands: Commands, menu: Query<Entity, With<PauseBackground>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
