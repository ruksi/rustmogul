use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::cleanup::{cleanup_for, CleanOnPlayMenuOpenExit};
use crate::gameplay::ActiveBoardId;
use crate::ledger::resource::Ledger;
use crate::states::{GlobalState, PlayMenuState};
use crate::ui;

pub struct PlayMenuPlugin;

impl Plugin for PlayMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayMenuState::Open), render_play_menu);
        app.add_systems(OnExit(PlayMenuState::Open), cleanup_for::<CleanOnPlayMenuOpenExit>);
    }
}

#[derive(Clone, Copy, EnumIter, Component)]
enum PlayMenuButton {
    Quit,
    DebugLedger,
}

impl PlayMenuButton {
    fn label(self) -> &'static str {
        match self {
            PlayMenuButton::Quit => "Quit",
            PlayMenuButton::DebugLedger => "Debug",
        }
    }
}

fn render_play_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ui::nodes::menu_container())
        .insert(CleanOnPlayMenuOpenExit)
        .with_children(|parent| {
            let font = asset_server.load(ui::fonts::BUTTON);
            for button in PlayMenuButton::iter() {
                spawn_button(parent, &font, button);
            }
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: &Handle<Font>, button: PlayMenuButton) {
    parent
        .spawn((
            ButtonBundle { ..ui::buttons::menu() },
            (match button {
                PlayMenuButton::Quit => On::<Pointer<Click>>::run(
                    |mut next_global_state: ResMut<NextState<GlobalState>>| {
                        next_global_state.set(GlobalState::InMainMenu);
                    },
                ),
                PlayMenuButton::DebugLedger => On::<Pointer<Click>>::run(
                    |ledger: Res<Ledger>, active_board: Res<ActiveBoardId>| {
                        debug!("{:?}", active_board);
                        debug!("{:?}", ledger);
                    },
                ),
            }),
        ))
        .with_children(|parent| {
            parent.spawn(ui::buttons::ButtonTextBundle::simple(button.label(), font, 36.0));
        });
}
