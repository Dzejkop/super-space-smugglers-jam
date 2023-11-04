use crate::contracts::Contract;
use crate::prelude::*;

static mut GAME: Option<Game> = None;

pub unsafe fn get() -> &'static Game {
    GAME.get_or_insert_with(Game::init)
}

pub unsafe fn get_mut() -> &'static mut Game {
    GAME.get_or_insert_with(Game::init)
}

pub fn tic() {
    let game = unsafe { get_mut() };

    game.time += DT * game.speed.to_speed();
}

pub struct Game {
    pub time: f32,
    pub speed: GameSpeed,
    pub fuel: f32,
    pub money: u32,
    pub tickets: u32,

    // Manouver mode stuff
    pub manouver_mode: bool,
    pub manouver_dv: Vec2,
    pub manouver_fuel: f32,

    // Contracts stuff
    pub contracts: Vec<Contract>,
    pub selected_contract: Option<usize>,
    pub cargo_hold: [Option<Contract>; 3],
}

impl Game {
    pub fn init() -> Self {
        Self {
            time: 0.0,
            speed: GameSpeed::Stop,
            fuel: 1.0,
            money: 10,
            tickets: 0,
            manouver_mode: false,
            manouver_dv: Vec2::ZERO,
            manouver_fuel: 0.0,
            contracts: vec![Contract {
                planet: 1,
                destination: 2,
                cargo: Cargo::Crabs,
                reward: 20,
            }],
            selected_contract: None,
            cargo_hold: [
                Some(Contract {
                    planet: 1,
                    destination: 3,
                    cargo: Cargo::Crabs,
                    reward: 20,
                }),
                None,
                None,
            ],
        }
    }

    pub fn steps(&self) -> u32 {
        match self.speed {
            GameSpeed::Stop => 0,
            GameSpeed::Normal => 1,
            GameSpeed::Fast => 2,
        }
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
    fn to_speed(&self) -> f32 {
        match self {
            GameSpeed::Stop => 0.0,
            GameSpeed::Normal => 1.0,
            GameSpeed::Fast => 2.0,
        }
    }
}
