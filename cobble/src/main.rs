use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod cleanup;
mod control;
mod gameplay;
mod hud;
mod ledger;
mod main_menu;
mod play_menu;
mod states;
mod test_utils;
mod timers;
mod ui;

fn main() {
    let log_filter =
        "wgpu_core=error,wgpu_hal=error,naga=warn,bevy_mod_picking=warn,rustmogul_cobble=debug";
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cobble".into(),
                        resolution: (800., 600.).into(),
                        // present_mode: PresentMode::AutoNoVsync, // AutoNoVsync = low picking latency
                        // fit_canvas_to_parent: true, // wasm: to resize to what is available
                        // prevent_default_event_handling: false, // wasm: don't prevent default keybindings
                        // window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: log_filter.into(),
                }),
        )
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(states::StatePlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(play_menu::PlayMenuPlugin)
        .add_plugins(control::ControlPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(hud::HudPlugin)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
