use bevy::prelude::*;

pub struct ButtonStyle {
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border: UiRect,
}

pub const BASIC_NORMAL: ButtonStyle = ButtonStyle {
    background_color: BackgroundColor(Color::rgb(0.75, 0.70, 0.60)),
    border_color: BorderColor(Color::rgb(0.20, 0.15, 0.15)),
    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(3.0), Val::Px(3.0)),
};
pub const BASIC_HOVER: ButtonStyle = ButtonStyle {
    background_color: BackgroundColor(Color::rgb(0.85, 0.80, 0.70)),
    border_color: BorderColor(Color::rgb(0.35, 0.30, 0.30)),
    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(2.0), Val::Px(4.0)),
};
pub const BASIC_PRESS: ButtonStyle = ButtonStyle {
    background_color: BackgroundColor(Color::rgb(0.55, 0.50, 0.35)),
    border_color: BorderColor(Color::rgb(0.20, 0.15, 0.15)),
    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(5.0), Val::Px(1.0)),
};
pub const BASIC_SELECTED: ButtonStyle = ButtonStyle {
    background_color: BackgroundColor(Color::rgb(0.65, 0.60, 0.50)),
    border_color: BorderColor(Color::rgb(0.30, 0.25, 0.25)),
    border: UiRect::new(Val::Px(3.0), Val::Px(3.0), Val::Px(4.0), Val::Px(2.0)),
};
