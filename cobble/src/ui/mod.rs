use bevy::prelude::*;

pub use plugin::UiPlugin;

pub mod buttons;
mod cursor;
pub mod fonts;
pub mod nodes;
mod plugin;

// pub const DEBUG_BACKGROUND: BackgroundColor = BackgroundColor(Color::rgba(1., 0., 1., 0.1));
pub const DEBUG_BACKGROUND: BackgroundColor = BackgroundColor(Color::rgba(0.36, 0.34, 0.35, 1.0));

pub const BLACK: Color = Color::rgb(0.20, 0.15, 0.15);
pub const WHITE: Color = Color::rgb(0.95, 0.95, 0.85);
pub const SOLO_TEXT_COLOR: Color = Color::rgb(0.95, 0.90, 0.65);
