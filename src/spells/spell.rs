use bevy::prelude::*;

#[derive(Debug)]
pub enum SpellError {
    InvalidTarget,
    NotEnoughMana,
    CooldownNotReady,
    UnknownError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellCategory {
    Projectile,
    Area,
    Buff,
    Summon,
}
pub trait Spell: Send + Sync + 'static {
    fn cast(&self, commands: &mut Commands, player_pos: IVec2, target: Option<IVec2>) -> Result<(), SpellError>;
    fn get_cooldown(&self) -> f32;
    fn get_mana_cost(&self) -> f32;
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_category(&self) -> SpellCategory;
    fn clone_box(&self) -> Box<dyn Spell>;
}
pub trait ProjectileSpell: Spell {}
pub trait AreaSpell: Spell {}
pub trait BuffSpell: Spell {}
pub trait SummonSpell: Spell {}
#[derive(Component, Clone)]
pub struct SpellBase {
    pub name: String,
    pub description: String,
    pub category: SpellCategory,
    pub cooldown: f32,
    pub mana_cost: f32,
    pub level: u32,
}
#[derive(Component, Clone)]
pub struct ProjectileSpellData {
    pub base: SpellBase,
    pub damage: f32,
    pub speed: f32,
    pub pierce_count: u32,
    pub spread: f32,
}

impl ProjectileSpellData {
    pub fn new(name: &str, description: &str, damage: f32, speed: f32, cooldown: f32, mana_cost: f32) -> Self {
        Self {
            base: SpellBase::new(name, description, SpellCategory::Projectile, cooldown, mana_cost),
            damage,
            speed,
            pierce_count: 0,
            spread: 0.0,
        }
    }
}
#[derive(Component, Clone)]
pub struct AreaSpellData {
    pub base: SpellBase,
    pub radius: f32,
    pub damage: f32,
    pub duration: f32,
}
