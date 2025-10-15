use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::{objects::{ShopNpc, Interaction, InteractionType}, resources::*};

/// margin to ensure the shop npcs spawn away from the edges of the screen
const SHOP_NPC_SPAWN_MARGIN: i32 = 2;

pub fn spawn_shop_npcs_on_rest_level(
    mut commands: Commands,
    terminal_query: Query<&Terminal>,
    camera_offset: Res<CameraOffset>,
    level: Res<Level>,
) {
    if level.as_ref() != &Level::Rest {
        return;
    }

    if let Ok(terminal) = terminal_query.single() {
        let terminal_size = terminal.size();
        let width = terminal_size[0] as i32;

        let min_x = -camera_offset.0.x + SHOP_NPC_SPAWN_MARGIN;
        let max_x = min_x + width - 1 - 2 * SHOP_NPC_SPAWN_MARGIN;

        let npc1_position = IVec2::new(
            min_x + (max_x - min_x) / 3,
            -camera_offset.0.y + SHOP_NPC_SPAWN_MARGIN,
        );
        let npc2_position = IVec2::new(
            min_x + 2 * (max_x - min_x) / 3,
            -camera_offset.0.y + SHOP_NPC_SPAWN_MARGIN,
        );

        commands.spawn((
            ShopNpc::new(npc1_position),
            Interaction::new(InteractionType::LeaderboardNpc),
            Transform::from_xyz(npc1_position.x as f32, npc1_position.y as f32, 0.0),
        ));
        commands.spawn((
            ShopNpc::new(npc2_position),
            Interaction::new(InteractionType::ShopNpc),
            Transform::from_xyz(npc2_position.x as f32, npc2_position.y as f32, 0.0),
        ));
    }
}
