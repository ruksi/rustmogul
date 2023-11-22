use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::cleanup::cleanup_for;
use crate::cleanup::CleanOnMainMenuExit;
use crate::states::GlobalState;
use crate::ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GlobalState::InMainMenu), render_main_menu);
        app.add_systems(OnExit(GlobalState::InMainMenu), cleanup_for::<CleanOnMainMenuExit>);
    }
}

#[derive(Clone, Copy, EnumIter, Component)]
enum MainMenuButton {
    Play,
    Options,
    Exit,
}

impl MainMenuButton {
    fn label(self) -> &'static str {
        match self {
            MainMenuButton::Play => "Play",
            MainMenuButton::Options => "Options",
            MainMenuButton::Exit => "Exit",
        }
    }
}

fn render_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // the main menu has no other cameras to piggyback on :(
    commands
        .spawn(Camera2dBundle::default())
        .insert(CleanOnMainMenuExit);

    commands
        .spawn(ui::nodes::menu_container())
        .insert(CleanOnMainMenuExit)
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Rustmogul",
                        TextStyle {
                            font: asset_server.load(ui::fonts::TITLE).clone(),
                            font_size: 110.0,
                            color: ui::BLACK,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Percent(4.0)),
                        ..default()
                    }),
                )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Rustmogul",
                            TextStyle {
                                font: asset_server.load(ui::fonts::TITLE).clone(),
                                font_size: 110.0,
                                color: ui::SOLO_TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            bottom: Val::Px(6.0),
                            right: Val::Px(4.0),
                            ..default()
                        }),
                    );
                });

            let font = asset_server.load(ui::fonts::BUTTON);
            for button in MainMenuButton::iter() {
                spawn_button(parent, &font, button);
            }
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: &Handle<Font>, button: MainMenuButton) {
    parent
        .spawn((
            ButtonBundle { ..ui::buttons::menu() },
            (match button {
                MainMenuButton::Play => {
                    On::<Pointer<Click>>::run(|mut global_state: ResMut<NextState<GlobalState>>| {
                        global_state.set(GlobalState::InPlay);
                    })
                }
                MainMenuButton::Options => On::<Pointer<Click>>::run(move || debug!("Options!")),
                MainMenuButton::Exit => {
                    On::<Pointer<Click>>::run(|mut exit_events: EventWriter<AppExit>| {
                        exit_events.send(AppExit)
                    })
                }
            }),
        ))
        .with_children(|parent| {
            parent.spawn(ui::buttons::ButtonTextBundle::simple(button.label(), font, 38.0));
        });
}
