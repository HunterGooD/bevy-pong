use crate::prelude::{ui::*, *};

pub fn label(txt: impl Into<String>) -> impl Bundle {
    let text = txt.into();
    (
        Text::new(text),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(BUTTON_TEXT_COLOR),
    )
}
