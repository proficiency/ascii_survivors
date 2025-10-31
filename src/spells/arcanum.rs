use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellType {
    Fireball,
    MagicMissile,
}

#[derive(Component, Clone)]
pub struct Arcanum {
    pub spells: Vec<SpellType>,
    pub mana: f32,
    pub max_mana: f32,
    pub mana_regen_rate: f32,
}

impl Arcanum {
    pub fn new() -> Self {
        Self {
            spells: Vec::new(),
            mana: 100.0,
            max_mana: 100.0,
            mana_regen_rate: 1.0,
        }
    }
    
    pub fn with_mana(max_mana: f32, regen_rate: f32) -> Self {
        Self {
            max_mana,
            mana: max_mana,
            mana_regen_rate: regen_rate,
            ..Self::new()
        }
    }
    
    pub fn learn_spell(&mut self, spell_type: SpellType) {
        if !self.spells.contains(&spell_type) {
            self.spells.push(spell_type);
        }
    }
    pub fn regenerate_mana(&mut self, delta_time: f32) {
        self.mana = (self.mana + delta_time).min(self.max_mana);
    }
    
    pub fn consume_mana(&mut self, amount: f32) -> bool {
        if self.mana >= amount {
            self.mana -= amount;
            true
        } else {
            false
        }
    }
}

impl Default for Arcanum {
    fn default() -> Self {
        Self::new()
    }
}
