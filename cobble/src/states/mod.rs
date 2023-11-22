use bevy::prelude::States;

pub use plugin::StatePlugin;

mod plugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GlobalState {
    #[default]
    InMainMenu,
    InPlay,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayState {
    #[default]
    None,
    Initialized,
    Plan,
    Execute,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayMenuState {
    #[default]
    Closed,
    Open,
}
