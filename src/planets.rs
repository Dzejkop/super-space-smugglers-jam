use crate::prelude::*;

static mut PLANETS: &mut [Planet] = &mut [
    // 0: The Sun
    Planet::new()
        .with_radius(120.0)
        .with_mass(0.5)
        .with_color(4),
    // 1: Mercury-ish
    Planet::new()
        .with_orbit(1000.0, 0.0002)
        .with_radius(30.0)
        .with_mass(0.1)
        .with_color(3),
    // 2: Venus-ish
    Planet::new()
        .with_orbit(2000.0, 0.00001)
        .with_radius(30.0)
        .with_mass(0.1)
        .with_color(4),
    // 3: Jupiter-ish
    Planet::new()
        .with_orbit(4000.0, 0.0002)
        .with_radius(60.0)
        .with_mass(0.22)
        .with_color(6),
    // 4: Europa
    Planet::moon_of(3)
        .with_orbit(300.0, 0.002)
        .with_radius(15.0)
        .with_mass(0.01)
        .with_color(9),
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
        let max_radius = if planet.parent.is_some() { 1.0 } else { 3.0 };

        circ(
            center.x,
            center.y,
            (camera.zoom * planet.radius).max(max_radius) as i32,
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
        planet.orbit_speed =
            orbits::orbital_period(central_mass, planet.radius);
    }

    unsafe {
        INIT = true;
    }
}
