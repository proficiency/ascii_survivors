use crate::{
    objects::{Interaction, *},
    resources::*,
};
use bevy::prelude::*;

const INTERACTION_KEY: KeyCode = KeyCode::KeyE;

pub fn interaction_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Player>,
    interaction_query: Query<(Entity, &Interaction, &GlobalTransform)>,
    mut commands: Commands,
    kill_count: Res<KillCount>,
    mut interaction_timer: ResMut<InteractionTimer>,
    time: Res<Time>,
) {
    interaction_timer.0.tick(time.delta());
    if keyboard_input.just_pressed(INTERACTION_KEY) && interaction_timer.0.finished() {
        if let Ok(player) = player_query.single() {
            let interaction_distance = 1.0;
            for (entity, interaction, transform) in interaction_query.iter() {
                let interaction_position = transform.translation();
                let player_position = player.world_position.as_vec2();
                let distance = interaction_position.truncate().distance(player_position);
                if distance <= interaction_distance {
                    match interaction.interaction_type {
                        InteractionType::Campfire => {
                            commands
                                .entity(entity)
                                .insert(Message::new("You feel rested.".to_string(), 2.0));
                        }
                        InteractionType::LeaderboardNpc => {
                            let message = format!("Enemies killed: {}", kill_count.enemies);
                            commands.entity(entity).insert(Message::new(message, 2.0));
                        }
                        InteractionType::ShopNpc => {
                            commands
                                .entity(entity)
                                .insert(Message::new("Hello, traveler!".to_string(), 2.0));
                        }
                    }
                    interaction_timer.0.reset();
                }
            }
        }
    }
}
