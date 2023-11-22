use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::ui;

pub fn update_button_visuals(
    mut interactions: Query<
        (
            Option<&PickingInteraction>,
            Option<&PickSelection>,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style,
        ),
        With<Button>,
    >,
) {
    for (interaction, selection, mut background_color, mut border_color, mut style) in
        &mut interactions
    {
        if let Some(selection) = selection {
            if selection.is_selected {
                *background_color = ui::buttons::styles::BASIC_SELECTED.background_color;
                *border_color = ui::buttons::styles::BASIC_SELECTED.border_color;
                style.border = ui::buttons::styles::BASIC_SELECTED.border;
                continue;
            }
        }
        match interaction {
            Some(PickingInteraction::Pressed) => {
                *background_color = ui::buttons::styles::BASIC_PRESS.background_color;
                *border_color = ui::buttons::styles::BASIC_PRESS.border_color;
                style.border = ui::buttons::styles::BASIC_PRESS.border;
            }
            Some(PickingInteraction::Hovered) => {
                *background_color = ui::buttons::styles::BASIC_HOVER.background_color;
                *border_color = ui::buttons::styles::BASIC_HOVER.border_color;
                style.border = ui::buttons::styles::BASIC_HOVER.border;
            }
            Some(PickingInteraction::None) | None => {
                *background_color = ui::buttons::styles::BASIC_NORMAL.background_color;
                *border_color = ui::buttons::styles::BASIC_NORMAL.border_color;
                style.border = ui::buttons::styles::BASIC_NORMAL.border;
            }
        }
    }
}
