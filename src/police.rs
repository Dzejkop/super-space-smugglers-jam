use crate::prelude::*;
use crate::ship::police_alternate_sprite;

const STARTING_POLICE_SPEED: f32 = 0.2;
const MAX_POLICE_SPEED: f32 = 0.75;
const MAX_SPEED_TIME: f32 = 60_000.0;

pub struct PoliceState {
    wanted: f32,
    dispatch_at: f32,
    deducation_at: f32,
    vehicles: Vec<PoliceVehicle>,
}

impl PoliceState {
    pub fn wanted(&self) -> f32 {
        self.wanted
    }

    pub fn increment_wanted_level(&mut self, t: f32) {
        self.wanted = (self.wanted + t).min(1.0);
    }
}

static mut STATE: PoliceState = PoliceState {
    wanted: 0.0,
    dispatch_at: 0.0,
    deducation_at: 0.0,
    vehicles: Vec::new(),
};

pub unsafe fn get() -> &'static PoliceState {
    &STATE
}

pub unsafe fn get_mut() -> &'static mut PoliceState {
    &mut STATE
}

pub fn tic(
    rng: &mut dyn RngCore,
    camera: &Camera,
    player: &Player,
    planets: &[Planet],
    game: &mut Game,
) -> bool {
    let state = unsafe { &mut STATE };

    // ---

    if keyp(keys::P, 0, 0) {
        msgs::add("Police MAX");
        state.wanted = 1.0;
    }

    if !player.is_caught {
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

            if state.wanted > 0.0
                && vehicles_in_pursuit < max_vehicles_in_pursuit
            {
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

                sfx(
                    1,
                    SfxOptions {
                        note: 1,
                        octave: 1,
                        duration: 5,
                        channel: 0,
                        ..Default::default()
                    },
                );
            }

            state.dispatch_at = game.time + rng.gen_range(10.0..25.0) * 1000.0;
        }
    }

    // ---

    let mut game_over = false;
    let mut bribe = None;

    for vehicle in &mut state.vehicles {
        let vehicle_pos = camera.world_to_screen(vehicle.pos);

        let vehicle_dir = match vehicle.behavior {
            PoliceVehicleBehavior::InPursuit => {
                let to_player = player.ship.pos - vehicle.pos;
                let player_vel = player.ship.vel;

                let distance = to_player.length();
                let time_to_impact = distance / player_vel.length();

                let player_pos_at_impact =
                    player.ship.pos + player_vel * time_to_impact;

                let to_player_at_impact = player_pos_at_impact - vehicle.pos;

                to_player_at_impact.normalize()
            }
            PoliceVehicleBehavior::Escaping { dir } => dir,
        };

        let speed = if game.time > MAX_SPEED_TIME {
            MAX_POLICE_SPEED
        } else {
            remap(
                game.time,
                (0.0, MAX_SPEED_TIME),
                (STARTING_POLICE_SPEED, MAX_POLICE_SPEED),
            )
        };

        let vehicle_vel = vehicle_dir * speed;

        let vehicle_engine_at = ShipSprite::police()
            .at(vehicle_pos)
            .rot(PI - vehicle_dir.angle_between(Vec2::Y))
            .scale(3.0 * camera.scale)
            .engine(true)
            .draw(Some(game));

        if player.is_caught {
            continue;
        }

        if let PoliceVehicleBehavior::InPursuit = &vehicle.behavior {
            if camera.scale < 0.15
                && time() % 1000.0 < 500.0
                && !game.manouver_mode
            {
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
                if bribe.is_none() && vehicle.collides_with(&player.ship) {
                    if game.credits == 0 {
                        game_over = true;
                    } else {
                        bribe = Some(rng.gen_range(1..10).min(game.credits));
                    }
                }
            }
        }
    }

    if let Some(bribe) = bribe {
        if bribe == game.credits {
            msgs::add("You *barely* bribed the patrol.");
        } else {
            msgs::add(format!("You bribed the patrol, $-{}k", bribe));
        }

        sfx(
            1,
            SfxOptions {
                note: 1,
                octave: 1,
                duration: 5,
                channel: 0,
                ..Default::default()
            },
        );

        for vehicle in &mut state.vehicles {
            if let PoliceVehicleBehavior::InPursuit = &vehicle.behavior {
                vehicle.behavior = PoliceVehicleBehavior::escaping(rng);
            }
        }

        game.credits -= bribe;
        state.dispatch_at = game.time + rng.gen_range(30.0..60.0) * 1000.0;
    }

    // ---

    state
        .vehicles
        .extract_if(|vehicle| {
            if let PoliceVehicleBehavior::Escaping { .. } = &vehicle.behavior {
                vehicle.pos.x < -50000.0
                    || vehicle.pos.y < -50000.0
                    || vehicle.pos.x > 50000.0
                    || vehicle.pos.y > 50000.0
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

        sfx(
            1,
            SfxOptions {
                note: 1,
                octave: 1,
                duration: 5,
                channel: 0,
                ..Default::default()
            },
        );
    }

    game_over
}

struct PoliceVehicle {
    pub pos: Vec2,
    pub behavior: PoliceVehicleBehavior,
}

impl PoliceVehicle {
    fn rand(rng: &mut dyn RngCore) -> Self {
        Self {
            pos: vec2(
                rng.gen_range(-5000.0..5000.0),
                rng.gen_range(-5000.0..5000.0),
            ),
            behavior: PoliceVehicleBehavior::InPursuit,
        }
    }

    fn collides_with(&self, player: &Ship) -> bool {
        self.pos.distance(player.pos) <= 90.0
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
