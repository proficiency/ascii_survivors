use crate::{objects::*, resources::*};
use bevy::prelude::*;
use rand::prelude::*;

pub fn campfire_animation_system(
    time: Res<Time>,
    level: Res<Level>,
    mut campfire_query: Query<&mut Campfire>,
    mut commands: Commands,
) {
    if level.as_ref() != &Level::Rest {
        return;
    }

    for mut campfire in campfire_query.iter_mut() {
        campfire.update(&time);
        campfire.ember_spawn_timer.tick(time.delta());

        if campfire.ember_spawn_timer.finished() {
            let mut rng = rand::rng();

            let velocity_x = rng.random_range(-1..=1);
            let velocity_y = -2;
            let velocity = IVec2::new(velocity_x, velocity_y);

            let lifetime = rng.random_range(0.01..0.065);
            let ember_position = campfire.position + IVec2::new(0, 3);

            commands.spawn((
                Ember::new(ember_position, velocity, lifetime),
                Transform::default(),
            ));
        }
    }
}
