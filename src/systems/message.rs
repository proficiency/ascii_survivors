use crate::objects::message::Message;
use crate::resources::CameraOffset;
use bevy::prelude::*;
use bevy_ascii_terminal::{GridSize, Terminal, string::TerminalString};

pub fn render_message_system(
    mut commands: Commands,
    mut message_query: Query<(Entity, &mut Message, &GlobalTransform)>,
    mut terminal_query: Query<&mut Terminal>,
    time: Res<Time>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        for (entity, mut message, transform) in message_query.iter_mut() {
            message.timer.tick(time.delta());
            if message.timer.finished() {
                commands.entity(entity).remove::<Message>();
            } else {
                let position = transform.translation().truncate().as_ivec2() + camera_offset.0;
                let terminal_position = position + IVec2::new(1, -1);
                let size = terminal.size();

                if size.contains_point([terminal_position.x, terminal_position.y]) {
                    let mut text = message.text.clone();
                    let max_len = (size[0] as i32 - terminal_position.x).max(0) as usize;
                    text.truncate(max_len);
                    let message_string = TerminalString::from(text);
                    terminal.put_string(terminal_position, message_string);
                }
            }
        }
    }
}
