use bevy::prelude::*;

use crate::objects::{Health, PlayerTag};
use crate::spells::Arcanum;
use crate::upgrades::modifiers::{PlayerModifiers, SpellModifiers};
use crate::upgrades::*;

pub fn apply_upgrade(
    upgrade: &UpgradeDefinition,
    health: &mut Health,
    arcanum: &mut Arcanum,
    modifiers: &mut PlayerModifiers,
    player_upgrades: &mut PlayerUpgrades,
) {
    for effect in &upgrade.effects {
        match effect {
            UpgradeEffect::IncreaseMaxHealth(amount) => {
                health.max += amount;
                health.current = (health.current + amount).max(1.0);
                health.current = health.current.min(health.max);
            }
            UpgradeEffect::IncreaseMaxMana(amount) => {
                arcanum.max_mana += amount;
                arcanum.mana = (arcanum.mana + amount).min(arcanum.max_mana);
            }
            UpgradeEffect::IncreaseManaRegen(amount) => {
                arcanum.mana_regen_rate += amount;
            }
            UpgradeEffect::IncomingDamageMultiplier(multiplier) => {
                modifiers.incoming_damage_multiplier *= multiplier;
            }
            UpgradeEffect::CastCooldownMultiplier(multiplier) => {
                modifiers.cast_cooldown_multiplier *= multiplier;
            }
            UpgradeEffect::SpellDamageMultiplier { spell, multiplier } => match spell {
                None => modifiers.global_damage_multiplier *= multiplier,
                Some(s) => {
                    modifiers
                        .spell_modifiers
                        .entry(*s)
                        .or_insert_with(SpellModifiers::new)
                        .damage_multiplier *= multiplier;
                }
            },
            UpgradeEffect::SpellSpeedMultiplier { spell, multiplier } => match spell {
                None => modifiers.global_speed_multiplier *= multiplier,
                Some(s) => {
                    modifiers
                        .spell_modifiers
                        .entry(*s)
                        .or_insert_with(SpellModifiers::new)
                        .speed_multiplier *= multiplier;
                }
            },
            UpgradeEffect::SpellManaCostMultiplier { spell, multiplier } => match spell {
                None => modifiers.global_mana_cost_multiplier *= multiplier,
                Some(s) => {
                    modifiers
                        .spell_modifiers
                        .entry(*s)
                        .or_insert_with(SpellModifiers::new)
                        .mana_cost_multiplier *= multiplier;
                }
            },
            UpgradeEffect::ExtraProjectiles { spell, count } => match spell {
                None => modifiers.extra_projectiles += count,
                Some(s) => {
                    modifiers
                        .spell_modifiers
                        .entry(*s)
                        .or_insert_with(SpellModifiers::new)
                        .extra_projectiles += count;
                }
            },
            UpgradeEffect::LearnSpell(spell_type) => {
                arcanum.learn_spell(*spell_type);
            }
        }
    }

    if let Some(existing) = player_upgrades.acquired.iter_mut().find(|a| a.id == upgrade.id) {
        existing.stacks += 1;
    } else {
        player_upgrades.acquired.push(AcquiredUpgrade {
            id: upgrade.id,
            stacks: 1,
        });
    }
}

pub fn apply_selected_upgrade(
    mut events: EventReader<UpgradeSelectedEvent>,
    mut player_query: Query<
        (&mut Health, &mut Arcanum, &mut PlayerUpgrades),
        With<PlayerTag>,
    >,
    mut modifiers: ResMut<PlayerModifiers>,
) {
    for event in events.read() {
        if let Ok((mut health, mut arcanum, mut upgrades)) =
            player_query.single_mut()
        {
            apply_upgrade(&event.upgrade, &mut health, &mut arcanum, &mut modifiers, &mut upgrades);
        }
    }
}
