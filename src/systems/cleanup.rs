use crate::objects::Portal;
use crate::objects::Portal;
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

// despawn all portals
pub fn despawn_portals(mut commands: Commands, portal_query: Query<Entity, With<Portal>>) {
    for entity in portal_query.iter() {
        commands.entity(entity).despawn();
    }
}
