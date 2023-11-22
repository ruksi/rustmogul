use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct DeselectAll;

pub fn handle_deselect_all(
    mut selectables: Query<&mut PickSelection>,
    events: EventReader<DeselectAll>,
) {
    if !events.is_empty() {
        for mut selection in &mut selectables.iter_mut() {
            if selection.is_selected {
                selection.is_selected = false;
            }
        }
    }
}
