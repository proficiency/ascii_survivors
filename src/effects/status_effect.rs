use crate::resources::timers::DamageEffectTimer;
use bevy::prelude::*;

#[derive(Component)]
pub struct StatusEffect {
    pub color: Color,
}

pub fn update_status_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<DamageEffectTimer>,
    player_query: Query<Entity, With<StatusEffect>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished()
        && let Ok(player_entity) = player_query.single()
    {
        commands.entity(player_entity).remove::<StatusEffect>();
    }
}
