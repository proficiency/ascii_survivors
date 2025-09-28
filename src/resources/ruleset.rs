use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct Ruleset {
    pub enemy_health_modifier: f32,
    pub enemy_damage_modifier: f32,
    pub enemy_spawn_rate: f32,
    pub player_health_modifier: f32,
    pub player_damage_modifier: f32,
    pub player_speed_modifier: f32,

    pub enemies_slow_on_attack: bool,
    pub enemies_stun_on_attack: bool,
}

impl Ruleset {
    pub fn default() -> Self {
        Self {
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            enemy_spawn_rate: 1.0,
            player_health_modifier: 1.0,
            player_damage_modifier: 1.0,
            player_speed_modifier: 1.0,
            enemies_slow_on_attack: false,
            enemies_stun_on_attack: false,
        }
    }
}
