use crate::prelude::*;

#[derive(Clone, Default)]
pub struct Planet {
    pub pos: Vec2,

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
    pub const fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            radius: 0.0,
            mass: 0.0,
            color: 0,
            parent: None,
        }
    }

    pub const fn moon_of(parent: usize) -> Self {
        Self {
            parent: Some(parent),
            ..Self::new()
        }
    }

    pub const fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.pos.x = x;
        self.pos.y = y;
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

    pub fn update(&mut self) {
        //
    }

    pub fn collides_with(&self, obj: Vec2) -> bool {
        self.pos.distance(obj) <= self.radius
    }
}
