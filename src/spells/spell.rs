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

impl SpellBase {
    pub fn new(name: &str, description: &str, category: SpellCategory, cooldown: f32, mana_cost: f32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category,
            cooldown,
            mana_cost,
            level: 1,
        }
    }
}

#[derive(Component, Clone)]
pub struct ProjectileSpellData {
    pub base: SpellBase,
    pub damage: f32,
    pub speed: f32,
    pub pierce_count: u32,
    pub spread: f32,
}

impl Spell for ProjectileSpellData {
    fn cast(&self, commands: &mut Commands, player_pos: IVec2, target: Option<IVec2>) -> Result<(), SpellError> {
        if let Some(target_pos) = target {
            let direction = (target_pos - player_pos).as_vec2().normalize_or_zero();

            // todo: implement spread properly, for spells like spread shot
            /*let spread_angle = if self.spread > 0.0 {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                (rng.r#gen::<f32>() - 0.5) * self.spread
            } else {
                0.0
            };*/

            let rotated_direction = if spread_angle != 0.0 {
                let cos = spread_angle.cos();
                let sin = spread_angle.sin();
                Vec2::new(
                    direction.x * cos - direction.y * sin,
                    direction.x * sin + direction.y * cos,
                )
            } else {
                direction
            };

            commands.spawn((
                Projectile {
                    position: player_pos,
                    target: None,
                    damage: self.damage,
                    speed: self.speed,
                },
                Transform::from_xyz(player_pos.x as f32, player_pos.y as f32, 0.0),
            ));

            Ok(())
        } else {
            Err(SpellError::InvalidTarget)
        }
    }

    fn get_cooldown(&self) -> f32 {
        self.base.cooldown
    }

    fn get_mana_cost(&self) -> f32 {
        self.base.mana_cost
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_description(&self) -> &str {
        &self.base.description
    }

    fn get_category(&self) -> SpellCategory {
        self.base.category
    }

    fn clone_box(&self) -> Box<dyn Spell> {
        Box::new(self.clone())
    }
}

impl ProjectileSpell for ProjectileSpellData {}

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

#[derive(Clone)]
pub struct SpellTemplate {
    pub name: String,
    pub description: String,
    pub category: SpellCategory,
    pub create_fn: Box<dyn Fn() -> Box<dyn Spell>>,
}

impl SpellTemplate {
    pub fn new<F>(name: &str, description: &str, category: SpellCategory, create_fn: F) -> Self
    where
        F: Fn() -> Box<dyn Spell> + 'static,
    {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category,
            create_fn: Box::new(create_fn),
        }
    }

    pub fn create_instance(&self) -> Box<dyn Spell> {
        (self.create_fn)()
    }
}