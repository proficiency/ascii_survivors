use bevy::prelude::*;

// has an entity become despawnable?
#[derive(Component)]
pub struct Despawn;

// despawn all despawnable entities
pub fn despawn_entities(mut commands: Commands, despawn_query: Query<Entity, With<Despawn>>) {
    for entity in despawn_query.iter() {
        commands.entity(entity).despawn();
    }
}
