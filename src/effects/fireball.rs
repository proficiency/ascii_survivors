use bevy::prelude::*;
use crate::objects::enemy::Enemy;
use crate::objects::boss::Boss;
use crate::resources::camera::CameraOffset;
use crate::systems::cleanup::Despawn;
use bevy_ascii_terminal::Terminal;
use bevy_ascii_terminal::string::TerminalString;

#[derive(Component)]
pub struct Fireball {
    pub position: IVec2,
    pub direction: Vec2,
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: u32,
}

impl Fireball {
    pub fn new(position: IVec2, direction: Vec2, damage: f32, speed: f32) -> Self {
        Self {
            position,
            direction,
            damage,
            speed,
            lifetime: 3.0,
            max_lifetime: 3.0,
            size: 1,
        }
    }
}

pub fn update_fireballs(
    mut commands: Commands,
    mut fireball_query: Query<(Entity, &mut Fireball)>,
    time: Res<Time>,
    camera_offset: Res<CameraOffset>,
    terminal_query: Query<&Terminal>,
) {
    for (entity, mut fireball) in fireball_query.iter_mut() {
        fireball.lifetime -= time.delta_secs();
        
        if fireball.lifetime <= 0.0 {
            commands.entity(entity).insert(Despawn);
            continue;
        }
        
        let direction = fireball.direction;
        let speed = fireball.speed;
        fireball.position += (direction * speed * time.delta_secs()).as_ivec2();
        
        if let Ok(terminal) = terminal_query.single() {
            let terminal_size = terminal.size();
            let draw_position = fireball.position + camera_offset.0;
            
            if draw_position.x < -10 
                || draw_position.x > terminal_size[0] as i32 + 10
                || draw_position.y < -10
                || draw_position.y > terminal_size[1] as i32 + 10
            {
                commands.entity(entity).insert(Despawn);
            }
        }
    }
}

pub fn render_fireballs(
    fireball_query: Query<&Fireball>,
    mut terminal_query: Query<&mut Terminal>,
    camera_offset: Res<CameraOffset>,
) {
    if let Ok(mut terminal) = terminal_query.single_mut() {
        let terminal_size = terminal.size();
        
        for fireball in fireball_query.iter() {
            let world_position = fireball.position + camera_offset.0;
            let draw_position = IVec2::new(
                world_position.x,
                terminal_size[1] as i32 - 1 - world_position.y,
            );
            
            if draw_position.x >= 0
                && draw_position.x < terminal_size[0] as i32
                && draw_position.y >= 0 
                && draw_position.y < terminal_size[1] as i32
            {
                let intensity = fireball.lifetime / fireball.max_lifetime;
                let mut fireball_char = TerminalString::from("O");
                
                let r = 1.0;
                let g = intensity * 0.5;
                let b = 0.0;
                fireball_char.decoration.fg_color = Some(bevy::prelude::LinearRgba::from(Color::linear_rgb(r, g, b)));
                
                terminal.put_string([draw_position.x as usize, draw_position.y as usize], fireball_char);
                
                if fireball.size > 1 {
                    let particles = [
                        (IVec2::new(-1, 0), "o"),
                        (IVec2::new(1, 0), "o"),
                        (IVec2::new(0, -1), "o"),
                        (IVec2::new(0, 1), "o"),
                    ];
                    
                    for (offset, char) in &particles {
                        let particle_pos = draw_position + *offset;
                        if particle_pos.x >= 0 
                            && particle_pos.x < terminal_size[0] as i32
                            && particle_pos.y >= 0 
                            && particle_pos.y < terminal_size[1] as i32
                        {
                            let mut particle_char = TerminalString::from(*char);
                            let r = 1.0;
                            let g = intensity * 0.3;
                            let b = 0.0;
                            particle_char.decoration.fg_color = Some(bevy::prelude::LinearRgba::from(Color::linear_rgb(r, g, b)));
                            terminal.put_string([particle_pos.x as usize, particle_pos.y as usize], particle_char);
                        }
                    }
                }
            }
        }
    }
}

pub fn fireball_collision(
    mut commands: Commands,
    fireball_query: Query<(Entity, &Fireball)>,
    mut enemy_query: Query<(Entity, &mut Enemy), Without<Despawn>>,
    mut boss_query: Query<(Entity, &mut Boss), Without<Despawn>>,
) {
    for (fireball_entity, fireball) in fireball_query.iter() {
        for (enemy_entity, mut enemy) in enemy_query.iter_mut() {
            if fireball.position == enemy.position {
                enemy.health -= fireball.damage;
                
                if enemy.health <= 0.0 {
                    commands.entity(enemy_entity).insert(Despawn);
                }
                
                commands.entity(fireball_entity).insert(Despawn);
                break;
            }
        }
        
        for (boss_entity, mut boss) in boss_query.iter_mut() {
            for (segment_index, segment) in boss.segments.iter().enumerate() {
                if fireball.position == segment.position {
                    let is_defeated = boss.take_damage(fireball.damage, segment_index);
                    if is_defeated {
                        commands.entity(boss_entity).insert(Despawn);
                    }
                    
                    commands.entity(fireball_entity).insert(Despawn);
                    break;
                }
            }
        }
    }
}
