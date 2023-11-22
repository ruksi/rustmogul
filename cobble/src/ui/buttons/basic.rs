use bevy::prelude::*;

use crate::ui;

pub fn basic() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            // self
            border: ui::buttons::styles::BASIC_NORMAL.border,
            // children
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: ui::buttons::styles::BASIC_NORMAL.background_color,
        border_color: ui::buttons::styles::BASIC_NORMAL.border_color,
        ..default()
    }
}
