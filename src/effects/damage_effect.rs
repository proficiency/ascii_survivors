use crate::resources::timers::DamageEffectTimer;
use bevy::prelude::*;

#[derive(Component)]
pub struct DamageEffect;

pub fn update_damage_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<DamageEffectTimer>,
    player_query: Query<Entity, With<DamageEffect>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished()
        && let Ok(player_entity) = player_query.single()
    {
        commands.entity(player_entity).remove::<DamageEffect>();
    }
}
