use bevy::prelude::*;
use std::collections::HashMap;

use crate::spells::SpellType;

#[derive(Clone, Debug, Default)]
pub struct SpellModifiers {
    pub damage_multiplier: f32,
    pub speed_multiplier: f32,
    pub mana_cost_multiplier: f32,
    pub extra_projectiles: u32,
}

impl SpellModifiers {
    pub fn new() -> Self {
        Self {
            damage_multiplier: 1.0,
            speed_multiplier: 1.0,
            mana_cost_multiplier: 1.0,
            extra_projectiles: 0,
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct PlayerModifiers {
    pub global_damage_multiplier: f32,
    pub global_speed_multiplier: f32,
    pub global_mana_cost_multiplier: f32,
    pub extra_projectiles: u32,
    pub incoming_damage_multiplier: f32,
    pub cast_cooldown_multiplier: f32,
    pub spell_modifiers: HashMap<SpellType, SpellModifiers>,
}

impl Default for PlayerModifiers {
    fn default() -> Self {
        Self {
            global_damage_multiplier: 1.0,
            global_speed_multiplier: 1.0,
            global_mana_cost_multiplier: 1.0,
            extra_projectiles: 0,
            incoming_damage_multiplier: 1.0,
            cast_cooldown_multiplier: 1.0,
            spell_modifiers: HashMap::new(),
        }
    }
}

impl PlayerModifiers {
    pub fn get_spell_modifiers(&self, spell_type: SpellType) -> SpellModifiers {
        self.spell_modifiers
            .get(&spell_type)
            .cloned()
            .unwrap_or_else(SpellModifiers::new)
    }

    pub fn get_damage(&self, spell_type: SpellType, base_damage: f32) -> f32 {
        let mods = self.get_spell_modifiers(spell_type);
        base_damage * self.global_damage_multiplier * mods.damage_multiplier
    }

    pub fn get_speed(&self, spell_type: SpellType, base_speed: f32) -> f32 {
        let mods = self.get_spell_modifiers(spell_type);
        base_speed * self.global_speed_multiplier * mods.speed_multiplier
    }

    pub fn get_mana_cost(&self, spell_type: SpellType, base_cost: f32) -> f32 {
        let mods = self.get_spell_modifiers(spell_type);
        base_cost * self.global_mana_cost_multiplier * mods.mana_cost_multiplier
    }

    pub fn get_total_projectiles(&self, spell_type: SpellType) -> u32 {
        let mods = self.get_spell_modifiers(spell_type);
        1 + self.extra_projectiles + mods.extra_projectiles
    }
}
