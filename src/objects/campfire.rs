use bevy::prelude::*;

#[derive(Component)]
pub struct Campfire {
    pub position: IVec2,
    pub animation_timer: Timer,
    pub ember_spawn_timer: Timer,
    pub current_frame: usize,
    pub characters: Vec<char>,
    pub colors: Vec<Color>,
}

impl Campfire {
    pub fn new(position: IVec2) -> Self {
        let characters = vec!['*', 'o', 'O', '0'];
        let colors = vec![
            Color::linear_rgb(1.0, 0.5, 0.0),
            Color::linear_rgb(1.0, 0.6, 0.0),
            Color::linear_rgb(1.0, 0.4, 0.0),
            Color::linear_rgb(1.0, 0.7, 0.0),
        ];

        Self {
            position,
            animation_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            ember_spawn_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            current_frame: 0,
            characters,
            colors,
        }
    }

    pub fn get_current_visual(&self) -> (char, Color) {
        let char_index = self.current_frame % self.characters.len();
        let color_index = self.current_frame % self.colors.len();
        (self.characters[char_index], self.colors[color_index])
    }

    pub fn update(&mut self, time: &Time) {
        self.animation_timer.tick(time.delta());
        if self.animation_timer.finished() {
            self.current_frame =
                (self.current_frame + 1) % (self.characters.len() * self.colors.len());
        }
    }
}
