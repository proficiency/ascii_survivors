use crate::objects::message::Message;
use bevy::prelude::*;
use bevy_ascii_terminal::{Terminal, string::TerminalString};

pub fn render_message_system(
    mut commands: Commands,
    mut message_query: Query<(Entity, &mut Message, &GlobalTransform)>,
    mut terminal_query: Query<&mut Terminal>,
    time: Res<Time>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        for (entity, mut message, transform) in message_query.iter_mut() {
            message.timer.tick(time.delta());
            if message.timer.finished() {
                commands.entity(entity).remove::<Message>();
            } else {
                let position = transform.translation();
                let terminal_position = IVec2::new(
                    position.x as i32 + 1,
                    terminal.size()[1] as i32 - position.y as i32 - 3,
                );

                let mut text = message.text.clone();
                text.truncate(terminal.size()[0] as usize - terminal_position.x as usize);
                let message_string = TerminalString::from(text);
                terminal.put_string(terminal_position, message_string);
            }
        }
    }
}
