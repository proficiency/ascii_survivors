use crate::objects::ember::Ember;
use crate::resources::level::Level;
use bevy::prelude::*;

pub fn ember_animation_system(
    time: Res<Time>,
    level: Res<Level>,
    mut ember_query: Query<(Entity, &mut Ember)>,
    mut commands: Commands,
) {
    if level.as_ref() != &Level::Rest {
        return;
    }
    
    for (entity, mut ember) in ember_query.iter_mut() {
        ember.update(&time);
        
        if ember.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
