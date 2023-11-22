use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use gold::*;
use robot_details::*;
use shop::*;

use crate::cleanup::CleanOnPlayExit;
use crate::gameplay::ActiveBoardId;
use crate::ledger::reactions::{BoardSynchronized, GoldChanged, ShopChanged};
use crate::states::PlayState;

mod gold;
mod robot_details;
mod shop;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::Initialized), spawn_hud);

        app.add_systems(Update, update_gold_on_synchronize.run_if(on_event::<BoardSynchronized>()));
        app.add_systems(Update, update_gold_on_change.run_if(on_event::<GoldChanged>()));

        app.add_systems(Update, update_shop_on_synchronize.run_if(on_event::<BoardSynchronized>()));
        app.add_systems(Update, update_shop_on_change.run_if(on_event::<ShopChanged>()));

        app.add_systems(
            Update,
            update_robot_detail_display.run_if(resource_exists::<ActiveBoardId>()),
        );
    }
}

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .insert(Pickable::IGNORE)
        .insert(CleanOnPlayExit)
        .with_children(|parent| {
            spawn_gold_display(parent, &asset_server);
            spawn_shop_display(parent, &asset_server);
            spawn_robot_details(parent, &asset_server);
        });
}
