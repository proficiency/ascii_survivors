mod sound;

// infinitely scaling ascii souls game
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use std::path::PathBuf;
use crate::sound::SoundManager;

trait Upgrade {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn apply(&self, ruleset: &mut Ruleset);
}

#[derive(Debug)]
struct SpellUpgrade {

}

impl Upgrade for SpellUpgrade {
    fn name(&self) -> &str {
        "Spell Upgrade"
    }

    fn description(&self) -> &str {
        "Makes dat fireball supa hot. Mega effective"
    }

    fn apply(&self, ruleset: &mut Ruleset) {
        // eventually take a Spell struct and modify it, constrain based on the upgrade type
        ruleset.player_damage_modifier *= 1.2;
    }
}

#[derive(Debug, Default)]
struct Ruleset {
    pub enemy_health_modifier: f32,
    pub enemy_damage_modifier: f32,
    pub enemy_spawn_rate: f32,
    pub player_health_modifier: f32,
    pub player_damage_modifier: f32,
    pub player_speed_modifier: f32,

    pub enemies_slow_on_attack: bool,
    pub enemies_stun_on_attack: bool,
}

impl Ruleset {
    pub fn default() -> Self {
        Self {
            enemy_health_modifier: 1.0,
            enemy_damage_modifier: 1.0,
            enemy_spawn_rate: 1.0,
            player_health_modifier: 1.0,
            player_damage_modifier: 1.0,
            player_speed_modifier: 1.0,
            enemies_slow_on_attack: false,
            enemies_stun_on_attack: false,
        }
    }
}

struct GameState {
    pub ruleset: Ruleset,
    pub spawn_queue: Vec<i32>,
    pub store: Vec<i32>,
    pub sound_manager: SoundManager,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ruleset: Ruleset::default(),
            spawn_queue: Vec::new(),
            store: Vec::new(),
            sound_manager: SoundManager::new("assets/sfx/".into()).unwrap(), // todo: error handling
        }
    }

    pub fn constrain() {

    }

    pub fn update() {

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut state = GameState::new();
    state.sound_manager.play_sound("assets/sfx/45_Charge_05.wav".into(), -8.0).unwrap();
    commands.spawn((
        Terminal::new([12, 1]).with_string([0, 0], "Hello world!".fg(color::BLUE)),
        TerminalBorder::single_line(),
    ));
    commands.spawn(TerminalCamera::new());
}