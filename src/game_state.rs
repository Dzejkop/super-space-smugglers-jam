use crate::tic80::time;

pub struct Game {
    // Time in game world
    game_time: f32,
    prev_game_time: f32,

    // Program time for tracking purposes
    time: f32,
    prev_time: f32,

    pub game_speed: GameSpeed,

    // State
    pub manouver_mode: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameSpeed {
    Stop,
    Normal,
    Fast,
}

impl GameSpeed {
    pub fn to_speed(&self) -> f32 {
        match self {
            GameSpeed::Stop => 0.0,
            GameSpeed::Normal => 1.0,
            GameSpeed::Fast => 2.0,
        }
    }
}

static mut GAME: Option<Game> = None;

pub fn game_mut() -> &'static mut Game {
    unsafe { GAME.get_or_insert_with(Game::init) }
}

impl Game {
    pub fn init() -> Self {
        Self {
            game_time: 0.0,
            prev_game_time: 0.0,
            time: 0.0,
            prev_time: 0.0,
            game_speed: GameSpeed::Stop,
            manouver_mode: false,
        }
    }

    pub fn update(&mut self) {
        self.prev_time = self.time;
        self.time = time();

        let dt = self.time - self.prev_time;

        self.prev_game_time = self.game_time;
        self.game_time += dt * self.game_speed.to_speed();
    }

    pub fn time(&self) -> f32 {
        self.game_time
    }

    pub fn dt(&self) -> f32 {
        self.game_time - self.prev_game_time
    }

    pub fn is_paused(&self) -> bool {
        self.game_speed == GameSpeed::Stop
    }
}
