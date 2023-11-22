use bevy::prelude::*;

use crate::states::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GlobalState>();
        app.add_state::<PlayState>();
        app.add_state::<PlayMenuState>();
    }
}
