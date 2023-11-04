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

    for (idx, planet) in planets.iter().enumerate() {
        // Draw orbit
        if let Some(parent) = planet.parent {
            let o = camera.world_to_screen(planets[parent].pos).as_ivec2();

            circb(
                o.x,
                o.y,
                (planet.orbit_radius * camera.zoom) as i32,
                planet.color,
            );
        } else {
            let o = camera.world_to_screen(vec2(0.0, 0.0)).as_ivec2();

            circb(
                o.x,
                o.y,
                (planet.orbit_radius * camera.zoom) as i32,
                planet.color,
            );
        }

        // Draw planet
        let pos = camera.world_to_screen(planet.pos).as_ivec2();

        if idx != 0 && camera.zoom < 0.15 {
            circb(pos.x, pos.y, 8, planet.color);
        }

        circ(
            pos.x,
            pos.y,
            (camera.zoom * planet.radius) as i32,
            planet.color,
        );
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
