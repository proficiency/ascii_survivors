use crate::plugins::audio::{AudioChannelType, AudioCommand};
use bevy::prelude::*;

#[derive(Clone, Event)]
pub struct AudioEvent {
    pub channel: AudioChannelType,
    pub command: AudioCommand,
}
