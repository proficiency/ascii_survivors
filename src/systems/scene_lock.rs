use crate::{objects::Portal, resources::*};
use bevy::prelude::*;

pub fn update_scene_lock(
    level: Res<Level>,
    portal_query: Query<&Portal>,
    mut scene_lock: ResMut<SceneLock>,
) {
    let has_portal = !portal_query.is_empty();
    scene_lock.0 = matches!(level.as_ref(), Level::Rest) || has_portal;
}
