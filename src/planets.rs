use crate::prelude::*;

static mut PLANETS: Vec<Planet> = Vec::new();

pub fn init(mut planets: Vec<Planet>) {
    let central_mass = planets[0].mass;

    for planet in &mut planets {
        planet.orbit_speed = orbital_period(central_mass, planet.radius);
    }

    unsafe {
        PLANETS = planets;
    }
}

pub unsafe fn get() -> &'static [Planet] {
    &PLANETS
}

pub unsafe fn get_mut() -> &'static mut [Planet] {
    &mut PLANETS
}

pub fn tic(camera: &Camera) {
    let planets = unsafe { get() };

    for planet in planets {
        let orbit = if let Some(parent) = planet.parent {
            camera.world_to_screen(planets[parent].pos)
        } else {
            camera.world_to_screen(vec2(0.0, 0.0))
        };

        draw_orbit(
            orbit.x,
            orbit.y,
            planet.orbit_radius * camera.zoom,
            planet.orbit_phase,
        );

        // ---

        let center = camera.world_to_screen(planet.pos).as_ivec2();
        let min_radius = if planet.parent.is_some() { 1.0 } else { 2.0 };

        circ(
            center.x,
            center.y,
            (camera.zoom * planet.radius).max(min_radius) as i32,
            planet.color,
        );
    }
}

fn draw_orbit(x: f32, y: f32, r: f32, p: f32) {
    let offset = vec2(x, y);
    let steps = 64;

    for step in (0..steps).step_by(2) {
        let a1 = (step as f32) / (steps as f32);
        let a2 = (step as f32 + 1.0) / (steps as f32);

        let a1 = p + a1 * 2.0 * PI;
        let a2 = p + a2 * 2.0 * PI;

        let p1 = offset + vec2(a1.cos(), a1.sin()) * r;
        let p2 = offset + vec2(a2.cos(), a2.sin()) * r;

        line(p1.x, p1.y, p2.x, p2.y, 14);
    }
}

// Returns orbital period according to
// T = 2π √(r³/GM)
// where
// T = orbital period
// r = orbit radius
// G = gravitational constant
// M = mass of central body
fn orbital_period(central_mass: f32, radius: f32) -> f32 {
    const G: f32 = 6.6743e-11;

    0.0001 * 2.0 * PI * (radius * radius * radius / (G * central_mass)).sqrt()
}

pub mod galaxies {
    use std::f32::consts::TAU;

    use super::*;

    pub fn alpha() -> Vec<Planet> {
        let mut planets = Vec::new();

        planets
            .push(Planet::new().with_radius(80.0).with_mass(2.5).with_color(4));

        for p in 0..20 {
            let idx = planets.len();
            let p = p as f32;

            planets.push(
                Planet::new()
                    .with_orbit(1000.0 + p * 750.0, p)
                    .with_radius(25.0 + p)
                    .with_mass(0.75)
                    .with_color(9),
            );

            planets.push(
                Planet::moon_of(idx)
                    .with_orbit(250.0, 0.0)
                    .with_radius(5.0)
                    .with_mass(0.15)
                    .with_color(5),
            );
        }

        planets
    }

    pub fn beta() -> Vec<Planet> {
        let mut planets = vec![
            Planet::new().with_radius(70.0).with_mass(5.0).with_color(4),
            Planet::new()
                .with_radius(20.0)
                .with_mass(0.5)
                .with_orbit(1200.0, 0.0)
                .with_color(13),
            Planet::new()
                .with_radius(25.0)
                .with_mass(1.0)
                .with_orbit(1400.0, 1.0)
                .with_color(2),
            Planet::new()
                .with_radius(30.0)
                .with_mass(1.5)
                .with_orbit(1600.0, 2.0)
                .with_color(9),
            Planet::new()
                .with_radius(10.0)
                .with_mass(0.35)
                .with_orbit(3000.0, 3.0)
                .with_color(3),
            Planet::new()
                .with_radius(50.0)
                .with_mass(2.5)
                .with_orbit(4500.0, 0.0)
                .with_color(4),
            Planet::new()
                .with_radius(52.5)
                .with_mass(1.5)
                .with_orbit(6000.0, 0.0)
                .with_color(12),
            Planet::new()
                .with_radius(25.0)
                .with_mass(3.0)
                .with_orbit(12000.0, 0.0)
                .with_color(11),
        ];

        for p in 0..8 {
            let p = p as f32;
            let idx = planets.len();

            planets.push(
                Planet::new()
                    .with_radius(25.0)
                    .with_mass(3.5)
                    .with_orbit(12000.0, 2.0 * PI * (p / 8.0))
                    .with_color(11),
            );

            for m in 0..3 {
                let m = m as f32;

                planets.push(
                    Planet::moon_of(idx)
                        .with_radius(5.0)
                        .with_mass(0.5)
                        .with_orbit(500.0, 2.0 * PI * (m / 3.0))
                        .with_color(6),
                );
            }
        }

        planets
    }

    pub fn gamma() -> Vec<Planet> {
        vec![
            Planet::new().with_radius(70.0).with_mass(5.0).with_color(4),
            Planet::new()
                .with_radius(20.0)
                .with_mass(1.0)
                .with_orbit(1500.0, 0.0)
                .with_color(3),
            Planet::new()
                .with_radius(28.0)
                .with_mass(1.0)
                .with_orbit(2500.0, 0.0)
                .with_color(11),
            Planet::new()
                .with_radius(35.0)
                .with_mass(1.2)
                .with_orbit(2800.0, 2.0)
                .with_color(10),
            Planet::new()
                .with_radius(40.0)
                .with_mass(1.5)
                .with_orbit(2800.0, 4.0)
                .with_color(9),
            Planet::new()
                .with_radius(35.0)
                .with_mass(1.2)
                .with_orbit(3000.0, 6.0)
                .with_color(8),
            Planet::new()
                .with_radius(100.0)
                .with_mass(2.5)
                .with_orbit(5000.0, 8.0)
                .with_color(2),
            Planet::new()
                .with_radius(100.0)
                .with_mass(2.5)
                .with_orbit(10000.0, 10.0)
                .with_color(5),
            Planet::moon_of(7)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 0.0)
                .with_color(11),
            Planet::moon_of(7)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 1.0 / 3.0 * TAU)
                .with_color(11),
            Planet::moon_of(7)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 2.0 / 3.0 * TAU)
                .with_color(11),
            Planet::new()
                .with_radius(100.0)
                .with_mass(2.5)
                .with_orbit(10000.0, 10.0 + PI)
                .with_color(5),
            Planet::moon_of(11)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 0.0)
                .with_color(11),
            Planet::moon_of(11)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 1.0 / 3.0 * TAU)
                .with_color(11),
            Planet::moon_of(11)
                .with_radius(10.0)
                .with_mass(0.25)
                .with_orbit(1000.0, 2.0 / 3.0 * TAU)
                .with_color(11),
        ]
    }
}
