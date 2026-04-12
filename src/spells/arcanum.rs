use crate::objects::{Fireball, Projectile};
use crate::upgrades::PlayerModifiers;
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

#[allow(dead_code)]
impl Arcanum {
    pub fn new() -> Self {
        Self {
            spells: Vec::new(),
            mana: 100.0,
            max_mana: 100.0,
            mana_regen_rate: 600.0,
        }
    }

    pub fn learn_spell(&mut self, spell_type: SpellType) {
        if !self.spells.contains(&spell_type) {
            self.spells.push(spell_type);
        }
    }

    pub fn can_cast_spell(&self, spell_type: SpellType, modifiers: &PlayerModifiers) -> bool {
        let mana_cost = self.get_spell_mana_cost(spell_type, modifiers);
        self.mana >= mana_cost && self.spells.contains(&spell_type)
    }

    pub fn cast_spell(
        &mut self,
        commands: &mut Commands,
        spell_type: SpellType,
        player_pos: IVec2,
        targets: &[Entity],
        modifiers: &PlayerModifiers,
    ) -> Result<(), &'static str> {
        if targets.is_empty() {
            return Err("no targets available");
        }

        let mana_cost = self.get_spell_mana_cost(spell_type, modifiers);
        if !self.consume_mana(mana_cost) {
            return Err("not enough mana to cast spell");
        }

        let total = modifiers.get_total_projectiles(spell_type);
        let offsets = spread_offsets(total);

        match spell_type {
            SpellType::Fireball => {
                let damage = modifiers.get_damage(spell_type, 25.0);
                let speed = modifiers.get_speed(spell_type, 150.0);

                for (i, offset) in offsets.iter().enumerate() {
                    let target = targets[i % targets.len()];
                    commands.spawn((
                        Projectile {
                            position: player_pos + *offset,
                            target: Some(target),
                            target_last_position: None,
                            damage,
                            speed,
                            lifetime: 3.0,
                        },
                        Fireball,
                    ));
                }
                Ok(())
            }
            SpellType::MagicMissile => {
                let damage = modifiers.get_damage(spell_type, 15.0);
                let speed = modifiers.get_speed(spell_type, 125.0);

                for (i, offset) in offsets.iter().enumerate() {
                    let target = targets[i % targets.len()];
                    commands.spawn((Projectile {
                        position: player_pos + *offset,
                        target: Some(target),
                        target_last_position: None,
                        damage,
                        speed,
                        lifetime: 3.0,
                    },));
                }
                Ok(())
            }
        }
    }

    pub fn get_spell_mana_cost(&self, spell_type: SpellType, modifiers: &PlayerModifiers) -> f32 {
        let base_cost = match spell_type {
            SpellType::Fireball => 20.0,
            SpellType::MagicMissile => 15.0,
        };
        modifiers.get_mana_cost(spell_type, base_cost)
    }

    pub fn get_spell_name(&self, spell_type: SpellType) -> &'static str {
        match spell_type {
            SpellType::Fireball => "Fireball",
            SpellType::MagicMissile => "Magic Missile",
        }
    }

    pub fn regenerate_mana(&mut self, delta_time: f32) {
        self.mana = (self.mana + delta_time * self.mana_regen_rate).min(self.max_mana);
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

// fan projectiles out from center in case of multiple projectiles
pub fn spread_offsets(count: u32) -> Vec<IVec2> {
    if count <= 1 {
        return vec![IVec2::ZERO];
    }
    let half = count as i32 / 2;
    (0..count as i32)
        .map(|i| {
            let offset = i - half;
            println!("{}", IVec2::new(offset, offset.abs() % 2));
            IVec2::new(offset, offset.abs() % 2)
        })
        .collect()
}
