use crate::prelude::*;

pub const MAX_MANOUVER_LENGTH: f32 = 10.0;
pub const MANOUVER_SENSITIVITY: f32 = 0.04;

static mut GAME: Option<Game> = None;

pub unsafe fn get() -> &'static Game {
    GAME.get_or_insert_with(Game::init)
}

pub unsafe fn get_mut() -> &'static mut Game {
    GAME.get_or_insert_with(Game::init)
}

pub fn tic() {
    unsafe { get_mut() }.update();
}

pub struct Game {
    real_time: f32,
    world_time: f32,

    prev_real_time: f32,
    prev_world_time: f32,

    pub speed: GameSpeed,
    pub fuel: f32,
    pub money: u32,
    pub tickets: u32,

    // Manouver mode stuff
    pub manouver_mode: bool,
    pub manouver_dv: Vec2,
}

impl Game {
    pub fn init() -> Self {
        Self {
            real_time: 0.0,
            world_time: 0.0,
            prev_real_time: 0.0,
            prev_world_time: 0.0,
            speed: GameSpeed::Stop,
            fuel: 1.0,
            money: 10,
            tickets: 0,
            manouver_mode: false,
            manouver_dv: Vec2::ZERO,
        }
    }

    pub fn update(&mut self) {
        self.prev_real_time = self.real_time;
        self.real_time = time();

        let dt = self.real_time - self.prev_real_time;

        self.prev_world_time = self.world_time;
        self.world_time += dt * self.speed.to_speed();
    }

    pub fn time(&self) -> f32 {
        self.world_time
    }

    pub fn day(&self) -> u32 {
        (self.world_time / 250.0) as u32
    }

    pub fn fuel(&self) -> f32 {
        self.fuel
    }

    pub fn ufuel(&self) -> u32 {
        (self.fuel * 100.0) as u32
    }

    pub fn tickets(&self) -> u32 {
        self.tickets
    }

    pub fn dt(&self) -> f32 {
        self.world_time - self.prev_world_time
    }

    pub fn is_paused(&self) -> bool {
        self.speed == GameSpeed::Stop
    }
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
