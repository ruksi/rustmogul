use bevy::prelude::*;

use crate::ui::buttons::systems::update_button_visuals;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button_visuals);
    }
}
