use crate::{
    events::*,
    objects::{GridPosition, Interaction, InteractionType, Message, PlayerTag},
    resources::KillCount,
};
use bevy::prelude::*;

const INTERACTION_DISTANCE: f32 = 1.0;
const MESSAGE_DURATION: f32 = 2.0;

#[derive(Event)]
pub struct InteractionMessageEvent {
    pub entity: Entity,
    pub text: String,
}

pub fn interaction_system(
    mut interact_events: EventReader<InteractEvent>,
    mut message_events: EventWriter<InteractionMessageEvent>,
    player_query: Query<&GridPosition, With<PlayerTag>>,
    interaction_query: Query<(Entity, &Interaction, &GlobalTransform)>,
    kill_count: Res<KillCount>,
) {
    if interact_events.read().next().is_none() {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        interact_events.clear();
        return;
    };

    let player_position = player_pos.world.as_vec2();
    if let Some((entity, interaction_type)) =
        find_nearby_interaction(player_position, &interaction_query)
    {
        let message_text = message_text_for(interaction_type, &kill_count);
        message_events.write(InteractionMessageEvent {
            entity,
            text: message_text,
        });
    }

    interact_events.clear(); // drop any excess events now that we've handled ours
}

fn find_nearby_interaction(
    player_position: Vec2,
    interaction_query: &Query<(Entity, &Interaction, &GlobalTransform)>,
) -> Option<(Entity, InteractionType)> {
    interaction_query
        .iter()
        .find_map(|(entity, interaction, transform)| {
            let distance = transform.translation().truncate().distance(player_position);
            if distance <= INTERACTION_DISTANCE {
                Some((entity, interaction.interaction_type.clone()))
            } else {
                None
            }
        })
}

pub fn apply_interaction_messages(
    mut commands: Commands,
    mut message_events: EventReader<InteractionMessageEvent>,
) {
    for event in message_events.read() {
        commands
            .entity(event.entity)
            .insert(Message::new(event.text.clone(), MESSAGE_DURATION));
    }
}

fn message_text_for(interaction_type: InteractionType, kill_count: &KillCount) -> String {
    match interaction_type {
        InteractionType::Campfire => "You feel rested.".to_string(),
        InteractionType::LeaderboardNpc => format!("Enemies killed: {}", kill_count.enemies),
        InteractionType::ShopNpc => "Hello, traveler!".to_string(),
    }
}
