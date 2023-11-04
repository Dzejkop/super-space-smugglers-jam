use crate::prelude::*;
use crate::ship::police_alternate_sprite;

pub struct State {
    wanted: f32,
    dispatch_at: f32,
    deducation_at: f32,
    vehicles: Vec<PoliceVehicle>,
}

impl State {
    pub fn wanted(&self) -> f32 {
        self.wanted
    }
}

static mut STATE: State = State {
    wanted: 1.0,
    dispatch_at: 0.0,
    deducation_at: 0.0,
    vehicles: Vec::new(),
};

pub unsafe fn get() -> &'static State {
    &STATE
}

pub fn tic(
    rng: &mut dyn RngCore,
    camera: &Camera,
    player: &Ship,
    planets: &[Planet],
    game: &mut Game,
) -> bool {
    let state = unsafe { &mut STATE };

    // ---

    if game.time >= state.deducation_at {
        let was_above_zero = state.wanted > 0.0;

        state.wanted = (state.wanted - 0.03).max(0.0);
        state.deducation_at = game.time + rng.gen_range(1.0..4.0) * 1000.0;

        if state.wanted == 0.0 && was_above_zero {
            msgs::add("Police seems to have lost interest in you.");

            for vehicle in &mut state.vehicles {
                vehicle.behavior = PoliceVehicleBehavior::escaping(rng);
            }
        }
    }

    if game.time >= state.dispatch_at {
        let vehicles_in_pursuit = state
            .vehicles
            .iter()
            .filter(|vehicle| {
                matches!(vehicle.behavior, PoliceVehicleBehavior::InPursuit)
            })
            .count();

        let max_vehicles_in_pursuit = (5.0 * state.wanted).ceil() as usize;

        if state.wanted > 0.0 && vehicles_in_pursuit < max_vehicles_in_pursuit {
            let vehicles = if rng.gen_bool(0.5) { 1 } else { 2 };

            for _ in 0..vehicles {
                state.vehicles.push(PoliceVehicle::rand(rng));
            }

            msgs::add({
                let msgs = [
                    "Police vehicle detected!",
                    "Oh no, it's the police!",
                    "Oh noes, police comes!",
                    "Schnapps, it's the cops!",
                    "Dang it, it's the cops!",
                    "Police, damn!",
                    "Damn, space-police!",
                    "Hide, it's the police!",
                    "The arm of the law approaches!",
                    "Has someone ordered a police patrol?",
                ];

                *msgs.choose(rng).unwrap()
            });
        }

        state.dispatch_at = game.time + rng.gen_range(10.0..25.0) * 1000.0;
    }

    // ---

    let mut game_over = false;
    let mut bribe = None;

    for vehicle in &mut state.vehicles {
        let vehicle_pos = camera.world_to_screen(vehicle.pos);

        let vehicle_dir = match vehicle.behavior {
            PoliceVehicleBehavior::InPursuit => {
                (player.pos - vehicle.pos).normalize()
            }
            PoliceVehicleBehavior::Escaping { dir } => dir,
        };

        let vehicle_vel = vehicle_dir * 0.2;

        let vehicle_engine_at = ShipSprite::police()
            .at(vehicle_pos)
            .rot(PI - vehicle_dir.angle_between(Vec2::Y))
            .scale(3.0 * camera.zoom)
            .engine(true)
            .draw(Some(game));

        if let PoliceVehicleBehavior::InPursuit = &vehicle.behavior {
            if camera.zoom < 0.15 && game.time % 1000.0 < 500.0 {
                let color = if police_alternate_sprite() { 10 } else { 2 };

                circb(vehicle_pos.x as i32, vehicle_pos.y as i32, 8, color);
            }

            OverflowIndicator::police(vehicle_pos).draw();
        }

        for _ in 0..game.steps() {
            vehicle.pos += vehicle_vel * DT;

            particles::spawn_exhaust(
                camera.screen_to_world(vehicle_engine_at),
                -vehicle_vel,
            );

            if let PoliceVehicleBehavior::InPursuit = &vehicle.behavior {
                if bribe.is_none() && vehicle.collides_with(player) {
                    if game.money == 0 {
                        game_over = true;
                    } else {
                        bribe = Some(rng.gen_range(1..10).min(game.money));
                    }
                }
            }
        }
    }

    if let Some(bribe) = bribe {
        if bribe == game.money {
            msgs::add("You *barely* bribed the patrol.");
        } else {
            msgs::add(format!("You bribed the patrol, $-{}k", bribe));
        }

        for vehicle in &mut state.vehicles {
            if let PoliceVehicleBehavior::InPursuit = &vehicle.behavior {
                vehicle.behavior = PoliceVehicleBehavior::escaping(rng);
            }
        }

        game.money -= bribe;
        state.dispatch_at = game.time + rng.gen_range(30.0..60.0) * 1000.0;
    }

    // ---

    state
        .vehicles
        .extract_if(|vehicle| {
            if let PoliceVehicleBehavior::Escaping { .. } = &vehicle.behavior {
                vehicle.pos.x < -10000.0
                    || vehicle.pos.y < -10000.0
                    || vehicle.pos.x > 10000.0
                    || vehicle.pos.y > 10000.0
            } else {
                false
            }
        })
        .for_each(drop);

    // ---

    let killed_vehicles: Vec<_> = state
        .vehicles
        .extract_if(|vehicle| planets[0].collides_with(vehicle.pos))
        .collect();

    if !killed_vehicles.is_empty() {
        for vehicle in killed_vehicles {
            for _ in 0..8 {
                let pos = vehicle.pos
                    + vec2(
                        rng.gen_range(-4.0..=4.0),
                        rng.gen_range(-4.0..=4.0),
                    );

                let dir =
                    vec2(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));

                if let Some(dir) = dir.try_normalize() {
                    particles::spawn_exhaust(pos, dir);
                }
            }
        }

        msgs::add({
            let msgs = [
                "Burn, baby!",
                "That must've hurt, sunshine!",
                "Say hello to Icarus!",
            ];

            *msgs.choose(rng).unwrap()
        });
    }

    game_over
}

struct PoliceVehicle {
    pub pos: Vec2,
    pub behavior: PoliceVehicleBehavior,
}

impl PoliceVehicle {
    fn rand(rng: &mut dyn RngCore) -> Self {
        let w = 5.0 * WIDTH as f32;
        let h = 5.0 * HEIGHT as f32;

        let dx = rng.gen_range(200.0..1500.0);
        let dy = rng.gen_range(200.0..1500.0);

        let x = if rng.gen_bool(0.5) { -w - dx } else { w + dx };
        let y = if rng.gen_bool(0.5) { -h - dy } else { h + dy };

        Self {
            pos: vec2(x, y),
            behavior: PoliceVehicleBehavior::InPursuit,
        }
    }

    fn collides_with(&self, player: &Ship) -> bool {
        self.pos.distance(player.pos) <= 64.0
    }
}

enum PoliceVehicleBehavior {
    InPursuit,
    Escaping { dir: Vec2 },
}

impl PoliceVehicleBehavior {
    fn escaping(rng: &mut dyn RngCore) -> Self {
        let dir = loop {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);

            if let Some(dir) = vec2(x, y).try_normalize() {
                break dir;
            }
        };

        PoliceVehicleBehavior::Escaping { dir }
    }
}
