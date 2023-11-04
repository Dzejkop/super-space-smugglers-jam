use crate::prelude::*;

enum State {
    Cooldown {
        until: f32,
        just_initialized: bool,
    },
    Hunting {
        warning: PoliceWarning,
        vehicles: Vec<PoliceVehicle>,
    },
}

static mut STATE: Option<State> = None;

pub fn tic(rng: &mut dyn RngCore, camera: &Camera, player: &Ship, game: &Game) {
    let time = time();

    let state = unsafe {
        STATE.get_or_insert_with(|| State::Cooldown {
            // until: time + rng.gen_range(20.0..30.0), TODO
            until: 0.0,
            just_initialized: true,
        })
    };

    match state {
        State::Cooldown {
            until,
            just_initialized,
        } => {
            if time < *until {
                return;
            }

            let vehicles = if *just_initialized {
                1
            } else {
                rng.gen_range(1..=3)
            };

            *state = State::Hunting {
                warning: PoliceWarning::rand(rng, *just_initialized),
                vehicles: (0..vehicles)
                    .map(|_| PoliceVehicle::rand(rng))
                    .collect(),
            };
        }

        State::Hunting { warning, vehicles } => {
            warning.tic();

            for vehicle in vehicles {
                let (vehicle_x, vehicle_y) =
                    camera.world_to_screen(vehicle.pos.x, vehicle.pos.y);

                let vehicle_pos = vec2(vehicle_x, vehicle_y);
                let vehicle_dir = (player.pos - vehicle.pos).normalize();

                ShipSprite::police()
                    .at(vehicle_pos)
                    .rot(PI - vehicle_dir.angle_between(Vec2::Y))
                    .scale(3.0 * camera.zoom)
                    .engine(true)
                    .draw();

                OverflowIndicator::police(vehicle_pos).draw();

                if !game.is_paused() {
                    vehicle.pos += vehicle_dir;
                }
            }
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

        let dx = rng.gen_range(200.0..800.0);
        let dy = rng.gen_range(200.0..800.0);

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
    fn rand(rng: &mut dyn RngCore, just_initialized: bool) -> Self {
        let text = if just_initialized {
            "Oh no, police vehicle detected!"
        } else {
            let texts = [
                "Police vehicle detected!",
                "Oh no, it's the police!",
                "Oh noes, police comes!",
                "Schnapps, it's the cops!",
                "Damn it, it's the cops!",
                "Dang, it's the police!",
                "Hide, it's the police!",
                "The arm of the law approaches!",
            ];

            *texts.choose(rng).unwrap()
        };

        Self {
            text,
            offset: 0.0,
            state: PoliceWarningState::GoingUp,
        }
    }

    fn tic(&mut self) {
        if let PoliceWarningState::GoingUp
        | PoliceWarningState::Stationary { .. }
        | PoliceWarningState::GoingDown = &self.state
        {
            Text::new(self.text)
                .at(vec2(0.0, HEIGHT as f32) - vec2(0.0, self.offset))
                .draw();
        }

        match &mut self.state {
            PoliceWarningState::GoingUp => {
                self.offset += 0.33;

                if self.offset >= 10.0 {
                    self.state =
                        PoliceWarningState::Stationary { elapsed: 0.0 };
                }
            }

            PoliceWarningState::Stationary { elapsed } => {
                *elapsed += 1.0;

                if *elapsed > 90.0 {
                    self.state = PoliceWarningState::GoingDown;
                }
            }

            PoliceWarningState::GoingDown => {
                self.offset -= 0.66;

                if self.offset <= 0.0 {
                    self.state = PoliceWarningState::Completed;
                }
            }

            PoliceWarningState::Completed => {
                //
            }
        }
    }
}

enum PoliceWarningState {
    GoingUp,
    Stationary { elapsed: f32 },
    GoingDown,
    Completed,
}
