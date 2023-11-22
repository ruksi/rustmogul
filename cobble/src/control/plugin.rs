use bevy::prelude::*;

use crate::control::pointer_event::PointerEvent;
use crate::control::selection::{handle_deselect_all, DeselectAll};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PointerEvent>();
        app.add_event::<DeselectAll>();
        app.add_systems(Update, handle_deselect_all.run_if(on_event::<DeselectAll>()));
    }
}
