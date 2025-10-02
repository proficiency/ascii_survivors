use bevy::prelude::*;

#[derive(Component)]
pub struct Interaction {
    pub interaction_type: InteractionType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionType {
    Campfire,
    LeaderboardNpc,
    ShopNpc,
}

impl Interaction {
    pub fn new(interaction_type: InteractionType) -> Self {
        Self { interaction_type }
    }
}
