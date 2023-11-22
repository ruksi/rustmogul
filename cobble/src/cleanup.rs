use bevy::prelude::*;

#[derive(Component)]
pub struct CleanOnMainMenuExit;

#[derive(Component)]
pub struct CleanOnPlayExit;

#[derive(Component)]
pub struct CleanOnPlayMenuOpenExit;

pub fn cleanup_for<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
