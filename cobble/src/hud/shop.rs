use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use uuid::Uuid;

use crate::gameplay::{ActiveBoardId, RobotToken};
use crate::ledger::actions::RerollShop;
use crate::ledger::reactions::{BoardSynchronized, ShopChanged};
use crate::ledger::Robot;
use crate::ui;
use crate::ui::buttons::ButtonTextBundle;

#[derive(Component)]
pub struct ShopSlots;

pub fn spawn_shop_display(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                // self
                position_type: PositionType::Absolute,
                top: Val::Percent(2.0),
                left: Val::Percent(1.0),
                right: Val::Percent(1.0),
                height: Val::Vh(14.0),
                // children
                column_gap: Val::Percent(1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        // self
                        flex_basis: Val::Vw(15.0),
                        // children
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        column_gap: Val::Percent(10.0),
                        ..default()
                    },
                    // background_color: ui::DEBUG_BACKGROUND,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                height: Val::Vh(8.0),
                                width: Val::Vw(5.0),
                                ..ui::buttons::basic().style
                            },
                            ..ui::buttons::basic()
                        })
                        .with_children(|parent| {
                            parent.spawn(ButtonTextBundle::simple(
                                "PS",
                                &asset_server.load(ui::fonts::BUTTON),
                                18.0,
                            ));
                        });
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                height: Val::Vh(8.0),
                                width: Val::Vw(5.0),
                                ..ui::buttons::basic().style
                            },
                            ..ui::buttons::basic()
                        })
                        .with_children(|parent| {
                            parent.spawn(ButtonTextBundle::simple(
                                "LK",
                                &asset_server.load(ui::fonts::BUTTON),
                                18.0,
                            ));
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        // self
                        flex_grow: 1.0,
                        min_width: Val::Px(0.0),
                        // children
                        column_gap: Val::Percent(1.0),
                        ..default()
                    },
                    // background_color: ui::DEBUG_BACKGROUND,
                    ..default()
                })
                .insert(ShopSlots)
                .with_children(|_parent| {
                    // spawn_shop_slot_button(parent, font, &None);
                    // spawn_shop_slot_button(parent, font, &None);
                    // spawn_shop_slot_button(parent, font, &None);
                    // spawn_shop_slot_button(parent, font, &None);
                    // spawn_shop_slot_button(parent, font, &None);
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        // self
                        flex_basis: Val::Vw(15.0),
                        // children
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexEnd,
                        column_gap: Val::Percent(10.0),
                        ..ui::buttons::basic().style
                    },
                    // background_color: ui::DEBUG_BACKGROUND,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    height: Val::Vh(8.0),
                                    width: Val::Vw(5.0),
                                    ..ui::buttons::basic().style
                                },
                                ..ui::buttons::basic()
                            },
                            On::<Pointer<Click>>::run(
                                |mut actions: EventWriter<RerollShop>,
                                 active_board: Res<ActiveBoardId>| {
                                    actions.send(RerollShop { board_id: active_board.0 });
                                },
                            ),
                        ))
                        .with_children(|parent| {
                            parent.spawn(ButtonTextBundle::simple(
                                "RR",
                                &asset_server.load(ui::fonts::BUTTON),
                                18.0,
                            ));
                        });
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                height: Val::Vh(8.0),
                                width: Val::Vw(5.0),
                                ..ui::buttons::basic().style
                            },
                            ..ui::buttons::basic()
                        })
                        .with_children(|parent| {
                            parent.spawn(ButtonTextBundle::simple(
                                "UP",
                                &asset_server.load(ui::fonts::BUTTON),
                                18.0,
                            ));
                        });
                });

            spawn_shop_close_button(parent, &asset_server.load(ui::fonts::BUTTON));
        });
}

pub fn update_shop_on_synchronize(
    mut commands: Commands,
    mut reactions: EventReader<BoardSynchronized>,
    active_board: Res<ActiveBoardId>,
    shop_slots_query: Query<Entity, With<ShopSlots>>,
    asset_server: Res<AssetServer>,
) {
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        if let Ok(shop_slots) = shop_slots_query.get_single() {
            commands
                .entity(shop_slots)
                .despawn_descendants()
                .with_children(|parent| {
                    let font = asset_server.load(ui::fonts::BUTTON);
                    for slot in &reaction.shop {
                        spawn_shop_slot_button(parent, &font, slot, active_board.0);
                    }
                });
        }
    }
}

pub fn update_shop_on_change(
    mut commands: Commands,
    mut reactions: EventReader<ShopChanged>,
    active_board: Res<ActiveBoardId>,
    shop_slots_query: Query<Entity, With<ShopSlots>>,
    asset_server: Res<AssetServer>,
) {
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        if let Ok(shop_slots) = shop_slots_query.get_single() {
            commands
                .entity(shop_slots)
                .despawn_descendants()
                .with_children(|parent| {
                    let font = asset_server.load(ui::fonts::NAME);
                    for slot in &reaction.shop {
                        spawn_shop_slot_button(parent, &font, slot, active_board.0);
                    }
                });
        }
    }
}

pub fn spawn_shop_slot_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    slot: &Option<Robot>,
    board_id: Uuid,
) {
    let text = match slot {
        Some(robot) => robot.name(),
        None => "".to_string(),
    };
    // let robot_id = match slot {
    //     Some(robot) => Some(robot.id.clone()),
    //     None => None,
    // };

    let icon_background_color = match slot {
        Some(robot) => BackgroundColor(robot.color()),
        None => BackgroundColor(Color::rgba(0., 0., 0., 0.)),
    };

    parent
        .spawn((
            Name::from("ShopSlotButton"),
            RobotToken { robot: slot.clone() },
            ButtonBundle {
                style: Style {
                    // self
                    flex_grow: 1.0,
                    flex_basis: Val::Percent(20.0),
                    min_width: Val::Px(0.0),
                    // children
                    flex_direction: FlexDirection::Column,
                    ..ui::buttons::basic().style
                },
                ..ui::buttons::basic()
            },
            PickableBundle::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // self
                            flex_grow: 1.0,
                            flex_basis: Val::Percent(75.0),
                            min_height: Val::Px(8.0),
                            min_width: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            max_width: Val::Percent(100.0),
                            // children
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        // background_color: ui::DEBUG_BACKGROUND,
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                height: Val::Percent(40.0),
                                width: Val::Percent(25.0),
                                margin: UiRect::top(Val::Percent(3.0)),
                                ..default()
                            },
                            background_color: icon_background_color,
                            ..default()
                        },
                        Pickable::IGNORE,
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            flex_basis: Val::Percent(25.0),
                            min_height: Val::Px(8.0),
                            min_width: Val::Px(0.0),
                            max_width: Val::Percent(100.0),
                            overflow: Overflow::clip_x(),
                            ..default()
                        },
                        // background_color: ui::DEBUG_BACKGROUND,
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    parent.spawn(ButtonTextBundle::simple(text, font, 16.0));
                });
        });
}

fn spawn_shop_close_button(parent: &mut ChildBuilder, font: &Handle<Font>) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(-4.0),
                    top: Val::Px(-4.0),
                    height: Val::Px(22.0),
                    width: Val::Px(16.0),
                    ..ui::buttons::basic().style
                },
                ..ui::buttons::basic()
            },
            On::<Pointer<Click>>::run(move || info!("Close pressed!")),
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn(ButtonTextBundle::simple("x", font, 16.0));
        });
}
