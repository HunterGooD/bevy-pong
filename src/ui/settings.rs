use crate::prelude::{ui::*, *};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuStates::Setting), setup_settings)
            .add_systems(OnExit(MenuStates::Setting), cleanup_settings);
    }
}

#[derive(Component)]
struct Setting;

fn setup_settings(mut commands: Commands) {
    info!("settings");
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
            GlobalZIndex(11),
            Setting,
        ))
        .with_child((
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            children![
                (
                    Text::new("Settings menu"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                ),
                default_button("Back", ButtonLabel::Back),
            ],
        ));
}

fn cleanup_settings(mut commands: Commands, menu: Query<Entity, With<Setting>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
