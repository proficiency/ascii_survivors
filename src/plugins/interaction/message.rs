use crate::objects::message::Message;
use crate::resources::{AsciiFrame, AsciiGrid};
use bevy::prelude::*;

pub fn render_message_system(
    mut commands: Commands,
    mut message_query: Query<(Entity, &mut Message, &GlobalTransform)>,
    mut frame: ResMut<AsciiFrame>,
    grid: Res<AsciiGrid>,
    time: Res<Time>,
) {
    for (entity, mut message, transform) in message_query.iter_mut() {
        message.timer.tick(time.delta());
        if message.timer.finished() {
            commands.entity(entity).remove::<Message>();
        } else {
            let position = transform.translation();
            let terminal_position = IVec2::new(
                position.x as i32 + 1,
                grid.grid_size.y as i32 - position.y as i32 - 3,
            );

            let mut text = message.text.clone();
            let max_width = (grid.grid_size.x as i32 - terminal_position.x).max(0) as usize;
            text.truncate(max_width);
            frame.put_string(terminal_position, text.as_str(), Color::WHITE, Color::NONE);
        }
    }
}
