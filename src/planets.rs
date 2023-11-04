use crate::prelude::*;

static mut PLANETS: &mut [Planet] = &mut [
    // 0: Sun
    Planet::new().with_radius(70.0).with_mass(5.0).with_color(4),
    // 1: Mercury-ish
    Planet::new()
        .with_radius(4.0)
        .with_mass(0.5)
        .with_orbit(1200.0)
        .with_color(13),
    // 2: Venus-ish
    Planet::new()
        .with_radius(5.0)
        .with_mass(1.0)
        .with_orbit(1450.0)
        .with_color(2),
    // 3: Earth-ish
    Planet::new()
        .with_radius(6.3)
        .with_mass(1.0)
        .with_orbit(1600.0)
        .with_color(9),
    // 4: Mars-ish
    Planet::new()
        .with_radius(3.3)
        .with_mass(0.1)
        .with_orbit(1880.0)
        .with_color(3),
    // 5: Jupiter-ish
    Planet::new()
        .with_radius(69.9)
        .with_mass(5.0)
        .with_orbit(4000.0)
        .with_color(4),
    // 6: Saturn-ish
    Planet::new()
        .with_radius(58.2)
        .with_mass(5.0)
        .with_orbit(6000.0)
        .with_color(12),
    // 7: Uranus-ish
    Planet::new()
        .with_radius(25.3)
        .with_mass(3.0)
        .with_orbit(9000.0)
        .with_color(11),
    // 8: Neptun-ish
    Planet::new()
        .with_radius(24.6)
        .with_mass(3.0)
        .with_orbit(1300.0)
        .with_color(10),
];

static mut INIT: bool = false;

pub unsafe fn get() -> &'static [Planet] {
    if !INIT {
        init();
    }

    &PLANETS
}

pub unsafe fn get_mut() -> &'static mut [Planet] {
    if !INIT {
        init();
    }

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

        draw_orbit(orbit.x, orbit.y, planet.orbit_radius * camera.zoom);

        // ---

        let center = camera.world_to_screen(planet.pos).as_ivec2();
        let min_radius = if planet.parent.is_some() { 1.0 } else { 2.0 };

        circ(
            center.x,
            center.y,
            (camera.zoom * planet.radius * 2.5).max(min_radius) as i32,
            planet.color,
        );
    }
}

fn draw_orbit(x: f32, y: f32, r: f32) {
    let offset = vec2(x, y);
    let steps = 64;

    for step in (0..steps).step_by(2) {
        let a1 = (step as f32) / (steps as f32);
        let a2 = (step as f32 + 1.0) / (steps as f32);

        let a1 = a1 * 2.0 * PI;
        let a2 = a2 * 2.0 * PI;

        let p1 = offset + vec2(a1.cos(), a1.sin()) * r;
        let p2 = offset + vec2(a2.cos(), a2.sin()) * r;

        line(p1.x, p1.y, p2.x, p2.y, 14);
    }
}

fn init() {
    let central_mass = unsafe { PLANETS[0].mass };

    for planet in unsafe { PLANETS.iter_mut() } {
        planet.orbit_speed = orbital_period(central_mass, planet.radius);
    }

    unsafe {
        INIT = true;
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
