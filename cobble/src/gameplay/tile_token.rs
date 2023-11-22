use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::control::PointerEvent;
use crate::gameplay::board_token::BoardToken;
use crate::gameplay::ActiveBoardId;
use crate::ledger::reactions::TileCreated;

#[derive(Component, Debug)]
pub struct TileToken {
    pub x: i8,
    pub y: i8,
}

pub fn react_to_tile_created(
    mut commands: Commands,
    mut reactions: EventReader<TileCreated>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_query: Query<Entity, With<BoardToken>>,
    active_board: Res<ActiveBoardId>,
) {
    let board_entity = board_query.single();
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        spawn_tile(
            &mut commands,
            &mut meshes,
            &mut materials,
            board_entity,
            reaction.x,
            reaction.y,
        );
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    x: i8,
    y: i8,
) -> Entity {
    let tile_size = 0.5;
    let entity = commands
        .spawn((
            Name::from(format!("TileToken({},{})", x, y)),
            TileToken { x, y },
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane::from_size(tile_size))),
                material: materials.add(Color::rgb(0.1, 0.3, 0.2).into()),
                transform: Transform::from_xyz(
                    (tile_size * x as f32) + (tile_size / 2.),
                    0.0,
                    (tile_size * y as f32) + (tile_size / 2.),
                ),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Drop>>::send_event::<PointerEvent>(),
            // On::<Pointer<DragStart>>::target_component_mut::<Visibility>(|_event, visibility| {
            //     // TODO: do DragStart of the child robot if exists
            // }),
            // On::<Pointer<DragEnd>>::target_commands_mut(|_event, commands| {
            //     // TODO: do DragEnd of the child robot if exists
            // }),
        ))
        .set_parent(parent)
        .id();
    entity
}
