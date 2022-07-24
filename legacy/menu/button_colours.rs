use crate::settings;
use bevy::prelude::{Color, UiColor};

pub struct ButtonColours {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Default for ButtonColours {
    fn default() -> Self {
        ButtonColours {
            normal: Color::hex(settings::BUTTON_COL.split_at(1).1)
                .unwrap()
                .into(),
            hovered: Color::hex(settings::BUTTON_HOVER_COL.split_at(1).1)
                .unwrap()
                .into(),
        }
    }
}
