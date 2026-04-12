pub mod modifiers;

use std::collections::VecDeque;

use bevy::prelude::*;

pub use modifiers::PlayerModifiers;

use crate::spells::SpellType;

#[derive(Clone, Debug)]
pub enum UpgradeEffect {
    IncreaseMaxHealth(f32),
    IncreaseMaxMana(f32),
    IncreaseManaRegen(f32),
    IncomingDamageMultiplier(f32),
    CastCooldownMultiplier(f32),
    SpellDamageMultiplier {
        spell: Option<SpellType>,
        multiplier: f32,
    },
    SpellSpeedMultiplier {
        spell: Option<SpellType>,
        multiplier: f32,
    },
    SpellManaCostMultiplier {
        spell: Option<SpellType>,
        multiplier: f32,
    },
    ExtraProjectiles {
        spell: Option<SpellType>,
        count: u32,
    },
    LearnSpell(SpellType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpgradeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl UpgradeRarity {
    pub fn color(&self) -> Color {
        match self {
            UpgradeRarity::Common => Color::WHITE,
            UpgradeRarity::Uncommon => Color::linear_rgb(0.3, 0.8, 1.0),
            UpgradeRarity::Rare => Color::linear_rgb(1.0, 0.85, 0.0),
            UpgradeRarity::Epic => Color::linear_rgb(0.85, 0.84, 0.17),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpgradeDefinition {
    pub id: &'static str, // todo: since this is strictly used for comparison, we should probably compare hashes instead
    pub name: &'static str,
    pub description: &'static str,
    pub rarity: UpgradeRarity,
    pub effects: Vec<UpgradeEffect>,
    pub weight: f32,
    pub max_stacks: u32,
}

#[derive(Clone, Debug)]
pub struct AcquiredUpgrade {
    pub id: &'static str,
    pub stacks: u32,
}

#[derive(Component, Default, Clone, Debug)]
pub struct PlayerUpgrades {
    pub acquired: Vec<AcquiredUpgrade>,
}

impl PlayerUpgrades {
    pub fn stack_count(&self, id: &str) -> u32 {
        self.acquired
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.stacks)
            .unwrap_or(0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UpgradeSource {
    LevelUp,
    BossOrb,
}

#[derive(Resource)]
pub struct UpgradeSelectionState {
    pub options: Vec<UpgradeDefinition>,
    pub selected_index: usize,
    pub source: UpgradeSource,
}

#[derive(Resource, Default)]
pub struct PendingUpgradeSelections {
    pub queue: VecDeque<UpgradeSource>,
}

#[derive(Event)]
pub struct ShowUpgradeSelectionEvent {
    pub source: UpgradeSource,
}

#[derive(Event)]
pub struct UpgradeSelectedEvent {
    pub upgrade: UpgradeDefinition,
}
