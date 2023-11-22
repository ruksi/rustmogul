use bevy::prelude::*;
use bevy_mod_picking::picking_core::*;

use crate::ui;

#[derive(Bundle, Debug)]
pub struct ButtonTextBundle {
    text_bundle: TextBundle,
    pickable: Pickable,
}

impl Default for ButtonTextBundle {
    fn default() -> Self {
        Self { text_bundle: default(), pickable: Pickable::IGNORE }
    }
}

impl ButtonTextBundle {
    pub fn simple(text: impl Into<String>, font: &Handle<Font>, font_size: f32) -> Self {
        Self {
            text_bundle: TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle { font: font.clone(), font_size, color: ui::BLACK },
                )
                .with_no_wrap(),
                ..default()
            },
            ..default()
        }
    }
}
