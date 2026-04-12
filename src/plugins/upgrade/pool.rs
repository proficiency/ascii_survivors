use bevy::prelude::*;
use rand::RngExt;

use crate::spells::SpellType;
use crate::upgrades::*;

#[derive(Resource)]
pub struct UpgradePool {
    pub definitions: Vec<UpgradeDefinition>,
}

impl Default for UpgradePool {
    fn default() -> Self {
        Self {
            definitions: build_upgrade_pool(),
        }
    }
}

pub fn select_random_upgrades(
    pool: &UpgradePool,
    player_upgrades: &PlayerUpgrades,
    count: usize,
) -> Vec<UpgradeDefinition> {
    let mut rng = rand::rng();

    let eligible: Vec<&UpgradeDefinition> = pool
        .definitions
        .iter()
        .filter(|def| {
            if def.max_stacks > 0 && player_upgrades.stack_count(def.id) >= def.max_stacks {
                return false;
            }
            true
        })
        .collect();

    if eligible.is_empty() {
        return Vec::new();
    }

    let total_weight: f32 = eligible.iter().map(|d| d.weight).sum();
    if total_weight <= 0.0 {
        return Vec::new();
    }

    let mut selected = Vec::with_capacity(count);
    let mut used_ids: Vec<&str> = Vec::new();

    for _ in 0..count {
        let available: Vec<&&UpgradeDefinition> = eligible
            .iter()
            .filter(|d| !used_ids.contains(&d.id))
            .collect();

        if available.is_empty() {
            break;
        }

        let available_weight: f32 = available.iter().map(|d| d.weight).sum();
        let mut roll = rng.random_range(0.0..available_weight);

        for def in &available {
            roll -= def.weight;
            if roll <= 0.0 {
                used_ids.push(def.id);
                selected.push((**def).clone());
                break;
            }
        }
    }

    selected
}

#[rustfmt::skip]
fn build_upgrade_pool() -> Vec<UpgradeDefinition> {
    // we could choose to derive the weight directly from the rarity, but it'd be nice to have uniquely rare upgrades
    const COMMON_WEIGHT: f32 = 12.0;
    const UNCOMMON_WEIGHT: f32 = 6.0;
    const RARE_WEIGHT: f32 = 3.0;
    const EPIC_WEIGHT: f32 = 1.0;

    vec![
        // tiered upgrades
        UpgradeDefinition {
            id: "fireball_dmg_2",
            name: "Fireball II",
            description: "+25% Fireball Damage",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::SpellDamageMultiplier {
                spell: Some(SpellType::Fireball),
                multiplier: 1.25,
            }],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 1,
        },
        UpgradeDefinition {
            id: "fireball_dmg_3",
            name: "Fireball III",
            description: "+30% Fireball Damage",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::SpellDamageMultiplier {
                spell: Some(SpellType::Fireball),
                multiplier: 1.30,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 1,
        },

        UpgradeDefinition {
            id: "missile_dmg_2",
            name: "Magic Missile II",
            description: "+25% Magic Missile Damage",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::SpellDamageMultiplier {
                spell: Some(SpellType::MagicMissile),
                multiplier: 1.25,
            }],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 1,
        },
        UpgradeDefinition {
            id: "missile_dmg_3",
            name: "Magic Missile III",
            description: "+30% Magic Missile Damage",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::SpellDamageMultiplier {
                spell: Some(SpellType::MagicMissile),
                multiplier: 1.30,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 1,
        },

        UpgradeDefinition {
            id: "health_1",
            name: "Vitality I",
            description: "+10 Max Health",

            rarity: UpgradeRarity::Common,
            effects: vec![UpgradeEffect::IncreaseMaxHealth(10.0)],
            weight: COMMON_WEIGHT,
            max_stacks: 5,
        },
        UpgradeDefinition {
            id: "health_2",
            name: "Vitality II",
            description: "+30 Max Health",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::IncreaseMaxHealth(30.0)],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 3,
        },

        UpgradeDefinition {
            id: "mana_1",
            name: "Intelligence I",
            description: "+15 Max Mana",

            rarity: UpgradeRarity::Common,
            effects: vec![UpgradeEffect::IncreaseMaxMana(15.0)],
            weight: COMMON_WEIGHT,
            max_stacks: 5,
        },
        UpgradeDefinition {
            id: "mana_2",
            name: "Intelligence II",
            description: "+30 Max Mana",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::IncreaseMaxMana(30.0)],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 5,
        },

        UpgradeDefinition {
            id: "mana_regen_1",
            name: "Mana Flow I",
            description: "+50 Mana Regen",

            rarity: UpgradeRarity::Common,
            effects: vec![UpgradeEffect::IncreaseManaRegen(50.0)],
            weight: COMMON_WEIGHT,
            max_stacks: 5,
        },
        UpgradeDefinition {
            id: "mana_regen_2",
            name: "Mana Flow II",
            description: "+100 Mana Regen",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::IncreaseManaRegen(100.0)],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 3,
        },

        UpgradeDefinition {
            id: "mana_cost_1",
            name: "Discipline I",
            description: "-15% Mana Cost",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::SpellManaCostMultiplier {
                spell: None,
                multiplier: 0.85,
            }],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 4,
        },
        UpgradeDefinition {
            id: "mana_cost_2",
            name: "Discipline II",
            description: "-35% Mana Cost",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::SpellManaCostMultiplier {
                spell: None,
                multiplier: 0.65,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 2,
        },

        // single-tier upgrades
        UpgradeDefinition {
            id: "all_dmg",
            name: "Arcane Insight",
            description: "+15% All Spell Damage",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::SpellDamageMultiplier {
                spell: None,
                multiplier: 1.15,
            }],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 5,
        },

        UpgradeDefinition {
            id: "spell_speed",
            name: "Quickcast",
            description: "+20% All Spell Speed",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::SpellSpeedMultiplier {
                spell: None,
                multiplier: 1.2,
            }],
            weight: UNCOMMON_WEIGHT,
            max_stacks: 4,
        },

        UpgradeDefinition {
            id: "rapid_fire",
            name: "Arcane Mastery",
            description: "-20% Cast Cooldown",

            rarity: UpgradeRarity::Uncommon,
            effects: vec![UpgradeEffect::CastCooldownMultiplier(0.8)],
            weight: RARE_WEIGHT,
            max_stacks: 4,
        },

        UpgradeDefinition {
            id: "extra_proj",
            name: "Multicast",
            description: "+1 Extra Projectile",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::ExtraProjectiles {
                spell: None,
                count: 1,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 3,
        },

        UpgradeDefinition {
            id: "extra_fireball",
            name: "Evocation",
            description: "+1 Extra Fireball",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::ExtraProjectiles {
                spell: Some(SpellType::Fireball),
                count: 1,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 2,
        },

        UpgradeDefinition {
            id: "extra_magicmissile",
            name: "Arcane Battery",
            description: "+1 Extra Magic Missile",

            rarity: UpgradeRarity::Rare,
            effects: vec![UpgradeEffect::ExtraProjectiles {
                spell: Some(SpellType::MagicMissile),
                count: 1,
            }],
            weight: RARE_WEIGHT,
            max_stacks: 2,
        },

        UpgradeDefinition {
            id: "glass_cannon",
            name: "Glass Cannon",
            description: "+50% Outgoing Damage, +25% Incoming Damage",

            rarity: UpgradeRarity::Rare,
            effects: vec![
                UpgradeEffect::SpellDamageMultiplier {
                    spell: None,
                    multiplier: 1.5,
                },
                UpgradeEffect::IncomingDamageMultiplier(1.25),
            ],
            weight: RARE_WEIGHT,
            max_stacks: 2,
        },
    ]
}
