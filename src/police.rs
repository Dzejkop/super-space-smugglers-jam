use crate::prelude::*;
use crate::ship::police_alternate_sprite;

struct State {
    wanted: f32,
    dispatch_at: f32,
    deducation_at: f32,
    warning: Option<PoliceWarning>,
    vehicles: Vec<PoliceVehicle>,
}

static mut STATE: State = State {
    wanted: 1.0,
    dispatch_at: 0.0,
    deducation_at: 0.0,
    warning: None,
    vehicles: Vec::new(),
};

pub fn tic(rng: &mut dyn RngCore, camera: &Camera, player: &Ship, game: &Game) {
    let state = unsafe { &mut STATE };

    // ---

    if game.time() >= state.deducation_at {
        state.wanted = (state.wanted - 0.05).max(0.0);
        state.deducation_at = game.time() + rng.gen_range(1.0..4.0) * 1000.0;
    }

    if game.time() >= state.dispatch_at {
        let max_vehicles = (5.0 * state.wanted).ceil() as usize;

        if state.wanted > 0.0
            && state.warning.is_none()
            && state.vehicles.len() < max_vehicles
        {
            let vehicles = if rng.gen_bool(0.5) { 1 } else { 2 };

            for _ in 0..vehicles {
                state.vehicles.push(PoliceVehicle::rand(rng));
            }

            state.warning = Some(PoliceWarning::rand(rng));
        }

        state.dispatch_at = game.time() + rng.gen_range(10.0..25.0) * 1000.0;
    }

    // ---

    for idx in 0..3 {
        let active = match idx {
            0 => state.wanted > 0.0,
            1 => state.wanted >= 0.4,
            2 => state.wanted >= 0.8,
            _ => unreachable!(),
        };

        let almost_inactive = match idx {
            0 => (0.0..=0.1).contains(&state.wanted),
            1 => (0.4..=0.5).contains(&state.wanted),
            2 => (0.8..=0.9).contains(&state.wanted),
            _ => unreachable!(),
        };

        let sprite = if active {
            if almost_inactive {
                if game.time() % 1000.0 < 500.0 {
                    263
                } else {
                    262
                }
            } else {
                263
            }
        } else {
            262
        };

        Img::sprite_idx(sprite)
            .at(vec2(WIDTH as f32 - 4.0 - 20.0 + 10.0 * (idx as f32), 16.0))
            .scale(1.0)
            .draw();
    }

    // ---

    if let Some(warning) = &mut state.warning {
        if !warning.tic() {
            state.warning = None;
        }
    }

    for vehicle in &mut state.vehicles {
        let (vehicle_x, vehicle_y) =
            camera.world_to_screen(vehicle.pos.x, vehicle.pos.y);

        let vehicle_pos = vec2(vehicle_x, vehicle_y);
        let vehicle_dir = (player.pos - vehicle.pos).normalize();
        let vehicle_vel = vehicle_dir * 0.1;

        ShipSprite::police()
            .at(vehicle_pos)
            .rot(PI - vehicle_dir.angle_between(Vec2::Y))
            .scale(3.0 * camera.zoom)
            .engine(true)
            .draw();

        if camera.zoom < 0.15 && game.time() % 1000.0 < 500.0 {
            let color = if police_alternate_sprite() { 10 } else { 2 };

            circb(vehicle_x as i32, vehicle_y as i32, 8, color);
        }

        OverflowIndicator::police(vehicle_pos).draw();

        if !game.is_paused() {
            vehicle.pos += vehicle_vel * game.dt();
        }
    }
}

struct PoliceVehicle {
    pos: Vec2,
}

impl PoliceVehicle {
    fn rand(rng: &mut dyn RngCore) -> Self {
        let w = 5.0 * WIDTH as f32;
        let h = 5.0 * HEIGHT as f32;

        let dx = rng.gen_range(200.0..1500.0);
        let dy = rng.gen_range(200.0..1500.0);

        let x = if rng.gen_bool(0.5) { -w - dx } else { w + dx };
        let y = if rng.gen_bool(0.5) { -h - dy } else { h + dy };

        Self { pos: vec2(x, y) }
    }
}

struct PoliceWarning {
    text: &'static str,
    offset: f32,
    state: PoliceWarningState,
}

impl PoliceWarning {
    fn rand(rng: &mut dyn RngCore) -> Self {
        let texts = [
            "Police vehicle detected!",
            "Oh no, it's the police!",
            "Oh noes, police comes!",
            "Schnapps, it's the cops!",
            "Dang it, it's the cops!",
            "Damn, police!",
            "Damn, space-police!",
            "Hide, it's the police!",
            "The arm of the law approaches!",
        ];

        Self {
            text: texts.choose(rng).unwrap(),
            offset: 0.0,
            state: PoliceWarningState::GoingDown,
        }
    }

    fn tic(&mut self) -> bool {
        if let PoliceWarningState::GoingDown
        | PoliceWarningState::Stationary { .. }
        | PoliceWarningState::GoingUp = &self.state
        {
            Text::new(self.text)
                .at(vec2(0.0, -8.0) + vec2(0.0, self.offset))
                .draw();
        }

        match &mut self.state {
            PoliceWarningState::GoingDown => {
                self.offset += 0.33;

                if self.offset >= 10.0 {
                    self.state =
                        PoliceWarningState::Stationary { elapsed: 0.0 };
                }

                true
            }

            PoliceWarningState::Stationary { elapsed } => {
                *elapsed += 1.0;

                if *elapsed > 90.0 {
                    self.state = PoliceWarningState::GoingUp;
                }

                true
            }

            PoliceWarningState::GoingUp => {
                self.offset -= 0.66;

                if self.offset <= 0.0 {
                    self.state = PoliceWarningState::Completed;
                }

                true
            }

            PoliceWarningState::Completed => false,
        }
    }
}

enum PoliceWarningState {
    GoingDown,
    Stationary { elapsed: f32 },
    GoingUp,
    Completed,
}
