use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_cursor);
    }
}

#[allow(dead_code)]
fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let mut window = windows.single_mut();
        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

#[allow(dead_code)]
fn cycle_cursor_icon(
    mut windows: Query<&mut Window>,
    input: Res<Input<MouseButton>>,
    mut index: Local<usize>,
) {
    let mut window = windows.single_mut();

    const ICONS: &[CursorIcon] = &[CursorIcon::Default, CursorIcon::Hand, CursorIcon::Wait];

    if input.just_pressed(MouseButton::Left) {
        *index = (*index + 1) % ICONS.len();
    } else if input.just_pressed(MouseButton::Right) {
        *index = if *index == 0 {
            ICONS.len() - 1
        } else {
            *index - 1
        };
    }

    window.cursor.icon = ICONS[*index];
}

#[allow(dead_code)]
fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    // if mouse.pressed(MouseButton::Left) {
    //     info!("left mouse currently pressed");
    // }
    // if mouse.just_pressed(MouseButton::Left) {
    //     info!("left mouse just pressed");
    // }
    // if mouse.just_released(MouseButton::Left) {
    //     info!("left mouse just released");
    // }
}
