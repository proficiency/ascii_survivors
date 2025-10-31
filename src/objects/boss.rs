use bevy::prelude::*;

#[derive(Component)]
pub struct Boss {
    pub health: f32,
    pub max_health: f32,
    pub segments: Vec<BossSegment>,
    pub speed: f32,
    pub damage: f32,
    pub boss_type: BossType,
}

#[derive(Clone)]
pub struct BossSegment {
    pub position: IVec2,
    pub segment_type: SegmentType,
    pub character: char,
    pub color: Color,
}

#[derive(Clone)]
pub enum SegmentType {
    Regular,
    Weakspot { damage_multiplier: f32 },
    Invulnerable,
}

#[derive(Clone)]
pub enum BossType {
    Snake,
    Giant,
}

impl Boss {
    pub fn new(position: IVec2, boss_type: BossType) -> Self {
        let segments = match boss_type {
            BossType::Snake => create_snake_segments(position),
            BossType::Giant => create_giant_segments(position),
        };

        let (health, speed, damage) = match boss_type {
            BossType::Snake => (200.0, 0.8, 15.0),
            BossType::Giant => (300.0, 0.4, 20.0),
        };

        Self {
            health,
            max_health: health,
            segments,
            speed,
            damage,
            boss_type,
        }
    }

    pub fn get_head_position(&self) -> IVec2 {
        if let Some(head) = self.segments.first() {
            head.position
        } else {
            IVec2::new(0, 0)
        }
    }

    pub fn take_damage(&mut self, damage: f32, segment_index: usize) -> bool {
        let actual_damage = match &self.segments[segment_index].segment_type {
            SegmentType::Weakspot { damage_multiplier } => damage * damage_multiplier,
            _ => damage,
        };

        self.health -= actual_damage;
        self.health <= 0.0
    }
}

fn create_snake_segments(start_pos: IVec2) -> Vec<BossSegment> {
    let mut segments = Vec::new();

    segments.push(BossSegment {
        position: start_pos,
        segment_type: SegmentType::Weakspot {
            damage_multiplier: 1.5,
        },
        character: 'S',
        color: Color::linear_rgb(0.8, 0.2, 0.2),
    });

    for i in 1..12 {
        segments.push(BossSegment {
            position: start_pos + IVec2::new(-i, 0),
            segment_type: SegmentType::Regular,
            character: 's',
            color: Color::linear_rgb(0.6, 0.1, 0.1),
        });
    }

    segments
}

fn create_giant_segments(start_pos: IVec2) -> Vec<BossSegment> {
    let mut segments = Vec::new();

    segments.push(BossSegment {
        position: start_pos,
        segment_type: SegmentType::Weakspot {
            damage_multiplier: 1.8,
        },
        character: 'G',
        color: Color::linear_rgb(0.2, 0.8, 0.2),
    });

    segments.push(BossSegment {
        position: start_pos + IVec2::new(0, -1),
        segment_type: SegmentType::Regular,
        character: 'B',
        color: Color::linear_rgb(0.1, 0.6, 0.1),
    });

    segments.push(BossSegment {
        position: start_pos + IVec2::new(-1, -1),
        segment_type: SegmentType::Regular,
        character: '/',
        color: Color::linear_rgb(0.1, 0.6, 0.1),
    });

    segments.push(BossSegment {
        position: start_pos + IVec2::new(1, -1),
        segment_type: SegmentType::Regular,
        character: '\\',
        color: Color::linear_rgb(0.1, 0.6, 0.1),
    });

    segments.push(BossSegment {
        position: start_pos + IVec2::new(-1, 1),
        segment_type: SegmentType::Regular,
        character: '/',
        color: Color::linear_rgb(0.1, 0.6, 0.1),
    });

    segments.push(BossSegment {
        position: start_pos + IVec2::new(1, 1),
        segment_type: SegmentType::Regular,
        character: '\\',
        color: Color::linear_rgb(0.1, 0.6, 0.1),
    });

    segments
}
