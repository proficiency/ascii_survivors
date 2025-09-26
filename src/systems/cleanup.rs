use bevy::prelude::*;

/// A marker component for entities that should be despawned.
#[derive(Component)]
pub struct Despawn;

/// System that despawns all entities with the `Despawn` marker component.
pub fn despawn_entities(
    mut commands: Commands,
    despawn_query: Query<Entity, With<Despawn>>,
) {
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn();
    }
}