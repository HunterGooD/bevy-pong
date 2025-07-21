use crate::prelude::{ui::*, *};
use bevy::ecs::spawn::SpawnWith;

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

// TODO: smart actions for button
// pub enum ButtonAction {
//     ChangeGameState(GameStates),
//     ChangeMenuState(MenuStates),
//     AppExit,
//     Back,
// }
//
// pub struct InteractiveButton {
//     pub action: ButtonAction,
//     pub width: f32,
//     pub label: ButtonLabel,
//     pub text: String,
// }

pub fn default_button(in_text: impl Into<String>, label: ButtonLabel) -> impl Bundle {
    let button_colors = ButtonColors::default();
    let text = in_text.into();
    (
        Name::new(format!("Button {text}")),
        Button,
        label,
        BorderRadius::MAX,
        Node {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(button_colors.normal),
        button_colors,
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Text::new(text),
                TextFont::from_font_size(40.),
                TextColor(BUTTON_TEXT_COLOR),
            ));
        })),
    )
}
