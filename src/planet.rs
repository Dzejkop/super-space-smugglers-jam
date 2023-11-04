#[derive(Clone, Default)]
pub struct Planet {
    pub x: f32,
    pub y: f32,

    // Oribital characteristics
    pub orbit_radius: f32,
    pub orbit_speed: f32,

    pub radius: f32,
    pub mass: f32,
    pub color: u8,

    // Parent planet index
    pub parent: Option<usize>,
}

impl Planet {
    pub const fn base() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            radius: 0.0,
            mass: 0.0,
            color: 0,
            parent: None,
        }
    }

    pub const fn planet(
        x: f32,
        y: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        radius: f32,
        mass: f32,
        color: u8,
    ) -> Self {
        Self {
            x,
            y,
            orbit_radius,
            orbit_speed,
            radius,
            mass,
            color,
            parent: None,
        }
    }

    pub const fn moon_of(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    pub const fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub const fn with_orbit(mut self, radius: f32, speed: f32) -> Self {
        self.orbit_radius = radius;
        self.orbit_speed = speed;
        self
    }

    pub const fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub const fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub const fn with_color(mut self, color: u8) -> Self {
        self.color = color;
        self
    }
}
