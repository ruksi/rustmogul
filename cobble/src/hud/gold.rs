use bevy::prelude::*;

use crate::gameplay::ActiveBoardId;
use crate::ledger::reactions::{BoardSynchronized, GoldChanged};
use crate::ui;

#[derive(Component)]
pub struct GoldText;

pub fn spawn_gold_display(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load(ui::fonts::BUTTON),
                    font_size: 18.0,
                    color: ui::SOLO_TEXT_COLOR,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(6.0),
                right: Val::Px(6.0),
                ..default()
            }),
        )
        .insert(GoldText);
}

pub fn update_gold_on_synchronize(
    mut reactions: EventReader<BoardSynchronized>,
    mut query: Query<&mut Text, With<GoldText>>,
    active_board: Res<ActiveBoardId>,
) {
    let mut gold_text = query.single_mut();
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        gold_text.sections[0].value = format!("Gold: {}", reaction.gold);
    }
}

pub fn update_gold_on_change(
    mut reactions: EventReader<GoldChanged>,
    mut query: Query<&mut Text, With<GoldText>>,
    active_board: Res<ActiveBoardId>,
) {
    let mut gold_text = query.single_mut();
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        if reaction.gold > reaction.old_gold {
            debug!("Gold Increased!");
        } else if reaction.old_gold > reaction.gold {
            debug!("Gold Decreased!");
        } else {
            warn!("Gold Unchanged!");
        }
        gold_text.sections[0].value = format!("Gold: {}", reaction.gold);
    }
}
