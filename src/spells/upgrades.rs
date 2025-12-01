use bevy::prelude::*;

#[derive(Debug)]
pub enum UpgradeError {
    IncompatibleSpell,
    InvalidParameter,
    UnknownError,
}

pub trait SpellUpgrade: Send + Sync + 'static {
    fn apply(&self, arcanum: &mut Arcanum) -> Result<(), UpgradeError>;
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn clone_box(&self) -> Box<dyn SpellUpgrade>;
}

#[derive(Clone)]
pub struct DamageUpgrade {
    pub name: String,
    pub description: String,
    pub damage_multiplier: f32,
}

impl DamageUpgrade {
    pub fn new(damage_multiplier: f32) -> Self {
        Self {
            name: format!("Damage +{}%", (damage_multiplier - 1.0) * 100.0),
            description: format!("Increase spell damage by {}%", (damage_multiplier - 1.0) * 100.0),
            damage_multiplier,
        }
    }
}
