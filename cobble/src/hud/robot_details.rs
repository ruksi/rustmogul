use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use uuid::Uuid;

use crate::control::DeselectAll;
use crate::gameplay::{ActiveBoardId, RobotToken};
use crate::ledger::actions::BuyRobot;
use crate::ledger::Robot;
use crate::ui;
use crate::ui::buttons::ButtonTextBundle;

#[derive(Component)]
pub struct RobotDetailContainer;

#[derive(Component)]
pub struct RobotDetailName;

#[derive(Component)]
pub struct RobotDetailImage;

#[derive(Component)]
pub struct RobotDetailActions;

pub fn spawn_robot_details(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    // self
                    position_type: PositionType::Absolute,
                    width: Val::Percent(13.0),
                    height: Val::Percent(55.0),
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    // children
                    display: Display::None, // Display::Flex when shown
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: ui::DEBUG_BACKGROUND,
                z_index: ZIndex::Global(-1),
                ..default()
            },
            RobotDetailContainer,
            NoDeselect,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // self
                            width: Val::Percent(100.0),
                            height: Val::Percent(10.0),
                            // children
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            overflow: Overflow::clip_x(),
                            ..default()
                        },
                        background_color: ui::DEBUG_BACKGROUND,
                        ..default()
                    },
                    NoDeselect,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: asset_server.load(ui::fonts::NAME),
                                    font_size: 18.0,
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            ..default()
                        },
                        RobotDetailName,
                        NoDeselect,
                    ));
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // self
                            width: Val::Percent(100.0),
                            height: Val::Percent(45.0),
                            // children
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        background_color: ui::DEBUG_BACKGROUND,
                        ..default()
                    },
                    NoDeselect,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                ..default()
                            },
                            NoDeselect,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        min_width: Val::Percent(180.0),  // overridden on show
                                        min_height: Val::Percent(180.0), // overridden on show
                                        top: Val::Percent(-50.0),        // overridden on show
                                        left: Val::Percent(0.0),         // overridden on show
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::WHITE),
                                    ..default()
                                },
                                UiImage::new(
                                    // overridden on show
                                    asset_server.load("robots/appliance/microwave-01.png"),
                                ),
                                RobotDetailImage,
                                NoDeselect,
                            ));
                        });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // self
                            width: Val::Percent(100.0),
                            height: Val::Percent(25.0),
                            // children
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: ui::DEBUG_BACKGROUND,
                        ..default()
                    },
                    NoDeselect,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "STATS GO HERE",
                                TextStyle {
                                    font: asset_server.load(ui::fonts::BODY),
                                    font_size: 12.0,
                                    ..default()
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            ..default()
                        },
                        NoDeselect,
                    ));
                });

            parent.spawn((
                NodeBundle {
                    style: Style {
                        // self
                        width: Val::Percent(100.0),
                        height: Val::Percent(20.0),
                        // children
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: ui::DEBUG_BACKGROUND,
                    ..default()
                },
                RobotDetailActions,
                NoDeselect,
            ));
        });
}

pub fn spawn_buy_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    board_id: Uuid,
    robot: &Robot,
) {
    let robot_id = robot.id;
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Px(26.0),
                    width: Val::Px(48.0),
                    ..ui::buttons::basic().style
                },
                ..ui::buttons::basic()
            },
            On::<Pointer<Click>>::run(
                move |mut actions: EventWriter<BuyRobot>, mut events: EventWriter<DeselectAll>| {
                    actions.send(BuyRobot { board_id, robot_id });
                    // TODO: should deselect IF the buy happens...
                    // TODO: should actually just refresh the robot details after buy...
                    events.send(DeselectAll);
                },
            ),
            NoDeselect,
        ))
        .with_children(|parent| {
            parent.spawn(ButtonTextBundle::simple(
                "Buy",
                &asset_server.load(ui::fonts::BUTTON),
                18.0,
            ));
        });
}

pub fn update_robot_detail_display(
    mut commands: Commands,
    mut selectables: Query<
        (Entity, &mut PickSelection, Option<&Name>, Option<&Children>),
        Changed<PickSelection>,
    >,
    mut container_query: Query<Entity, With<RobotDetailContainer>>,
    mut name_query: Query<&mut Text, With<RobotDetailName>>,
    mut image_query: Query<(Entity, &mut UiImage), With<RobotDetailImage>>,
    mut actions_query: Query<Entity, With<RobotDetailActions>>,
    mut style_query: Query<&mut Style>,
    token_query: Query<&RobotToken>,
    active_board: Res<ActiveBoardId>,
    asset_server: Res<AssetServer>,
) {
    for (selectable_entity, selectable, selectable_name, children) in selectables.iter_mut() {
        if selectable.is_added() {
            // don't trigger rendering of robot details when anything selectable is created
            continue;
        }

        let container_entity = container_query.single_mut();
        let mut style = style_query.get_mut(container_entity).unwrap();

        if !selectable.is_selected {
            // hide robot details when nothing is selected
            // TODO: this should maybe be timed as a new selection might come in next frame
            style.display = Display::None;
            continue;
        }

        // find robot token in the selectable or its children
        let mut token: Option<&RobotToken> = None;
        if let Ok(my_token) = token_query.get(selectable_entity) {
            token = Some(my_token);
        };
        if token.is_none() && children.is_some() {
            if let Some(children) = children {
                for &child in children.iter() {
                    if let Ok(child_token) = token_query.get(child) {
                        token = Some(child_token);
                        break;
                    }
                }
            }
        }
        let Some(token) = token else {
            // we couldn't find any robot token to show
            continue;
        };
        let Some(robot) = &token.robot else {
            // the robot token is empty :(
            continue;
        };

        style.display = Display::Flex;

        // update the robot name
        let mut name = name_query.single_mut();
        name.sections[0].value = robot.name();

        // render appropriate actions
        let actions_entity = actions_query.single_mut();
        commands
            .entity(actions_entity)
            .despawn_descendants()
            .with_children(|parent| {
                if let Some(selectable_name) = selectable_name {
                    if selectable_name.contains("ShopSlot") {
                        spawn_buy_button(parent, &asset_server, active_board.0, &robot)
                    }
                    if selectable_name.contains("Token") {
                        // TODO: sell button
                    }
                };
            });

        // update the robot image
        let (image_entity, mut image) = image_query.single_mut();
        let mut image_style = style_query.get_mut(image_entity).unwrap();
        match robot.prototype_id.as_str() {
            "microwave" => {
                image.texture = asset_server.load("robots/appliance/microwave-01.png");
                image_style.min_width = Val::Percent(210.0);
                image_style.min_height = Val::Percent(210.0);
                image_style.left = Val::Percent(-82.5);
                image_style.top = Val::Percent(0.0);
            }
            "magic_missile" => {
                image.texture = asset_server.load("robots/arcane/magic-missile-01.png");
                image_style.min_width = Val::Percent(150.0);
                image_style.min_height = Val::Percent(150.0);
                image_style.left = Val::Percent(-25.0);
                image_style.top = Val::Percent(-25.0);
            }
            "barker" => {
                image.texture = asset_server.load("robots/military/barker-01.png");
                image_style.min_width = Val::Percent(180.0);
                image_style.min_height = Val::Percent(180.0);
                image_style.left = Val::Percent(-62.5);
                image_style.top = Val::Percent(-5.0);
            }
            _ => panic!("Unknown robot prototype, can't visualize: {}", robot.prototype_id),
        }
    }
}
