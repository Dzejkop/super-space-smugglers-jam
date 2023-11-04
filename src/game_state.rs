use crate::tic80::time;

pub struct Game {
    real_time: f32,
    world_time: f32,

    prev_real_time: f32,
    prev_world_time: f32,

    pub speed: GameSpeed,
    pub manouver_mode: bool,
    pub fuel: f32,
    pub money: f32,
    pub tickets: u32,
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
            real_time: 0.0,
            world_time: 0.0,
            prev_real_time: 0.0,
            prev_world_time: 0.0,
            speed: GameSpeed::Stop,
            manouver_mode: false,
            fuel: 100.0,
            money: 10000.0,
            tickets: 0,
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

    pub fn fuel(&self) -> u32 {
        self.fuel as u32
    }

    pub fn money_str(&self) -> String {
        format!("${}k", (self.money / 1000.0) as u32)
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
