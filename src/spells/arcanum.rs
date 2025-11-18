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

    pub fn cast_spell(
        &self,
        commands: &mut Commands,
        spell_type: SpellType,
        player_pos: IVec2,
        target: Option<Entity>,
    ) -> Result<(), &'static str> {
        match spell_type {
            SpellType::Fireball => {
                if target.is_some() {
                    commands.spawn((
                        crate::objects::projectile::Projectile {
                            position: player_pos,
                            target,
                            target_last_position: None,
                            damage: 25.0,
                            speed: 150.0,
                            lifetime: 3.0,
                            max_lifetime: 3.0,
                        },
                        crate::objects::projectile::Fireball,
                    ));
                    Ok(())
                } else {
                    Err("tried to cast Fireball on an invalid target")
                }
            }
            SpellType::MagicMissile => {
                if target.is_some() {
                    commands.spawn((crate::objects::projectile::Projectile {
                        position: player_pos,
                        target,
                        target_last_position: None,
                        damage: 15.0,
                        speed: 125.0,
                        lifetime: 3.0,
                        max_lifetime: 3.0,
                    },));
                    Ok(())
                } else {
                    Err("tried to cast Magic Missile on an invalid target")
                }
            }
        }
    }

    fn get_spell_mana_cost(&self, spell_type: SpellType) -> f32 {
        match spell_type {
            SpellType::Fireball => 20.0,
            SpellType::MagicMissile => 15.0,
        }
    }

    fn get_spell_name(&self, spell_type: SpellType) -> &'static str {
        match spell_type {
            SpellType::Fireball => "Fireball",
            SpellType::MagicMissile => "Magic Missile",
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
