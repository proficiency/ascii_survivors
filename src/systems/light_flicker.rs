use bevy::prelude::*;
use rand::{rng, Rng};

use crate::objects::{LightEmitter, LightFlicker};

pub fn light_flicker_system(
    time: Res<Time>,
    mut query: Query<(&mut LightEmitter, &mut LightFlicker)>,
) {
    let mut rng = rng();

    for (mut emitter, mut flicker) in query.iter_mut() {
        flicker.timer.tick(time.delta());

        if flicker.timer.finished() {
            flicker.target_intensity =
                rng.random_range(flicker.intensity_range.0..=flicker.intensity_range.1);
            flicker.target_radius =
                rng.random_range(flicker.radius_range.0..=flicker.radius_range.1);
        }

        let delta = time.delta_secs();
        emitter.intensity += (flicker.target_intensity - emitter.intensity)
            * flicker.lerp_speed
            * delta;
        emitter.radius += (flicker.target_radius - emitter.radius)
            * flicker.lerp_speed
            * delta;
    }
}
