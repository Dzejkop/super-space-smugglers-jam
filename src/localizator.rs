use crate::prelude::*;

pub struct Localizator {
    at: Vec2,
    ty: LocalizatorTy,
}

impl Localizator {
    pub fn player(at: Vec2) -> Self {
        Self {
            at,
            ty: LocalizatorTy::Player,
        }
    }

    pub fn police(at: Vec2) -> Self {
        Self {
            at,
            ty: LocalizatorTy::Police,
        }
    }

    pub fn contract(at: Vec2) -> Self {
        Self {
            at,
            ty: LocalizatorTy::Contract,
        }
    }

    pub fn draw(self) {
        unsafe {
            INDICATORS.push(self);
        }
    }

    fn draw_ex(self) {
        let Self { ty: id, at } = self;

        let color = match id {
            LocalizatorTy::Player => {
                if blink() {
                    return;
                } else {
                    5
                }
            }

            LocalizatorTy::Police => {
                if blink() {
                    2
                } else {
                    10
                }
            }

            LocalizatorTy::Contract => {
                if blink() {
                    3
                } else {
                    4
                }
            }
        };

        let at_2 = at.clamp(
            vec2(2.0, 2.0),
            vec2(WIDTH as f32 - 2.0, HEIGHT as f32 - 2.0),
        );

        let dir = (at - at_2).normalize();

        let arrow = [
            (vec2(0.0, 8.0), vec2(0.0, 1.0)),
            (vec2(0.0, 2.0), vec2(-3.0, 4.0)),
            (vec2(0.0, 2.0), vec2(3.0, 4.0)),
        ];

        let transform = |v: Vec2| -> Vec2 {
            rotate(at_2 + v, at_2, PI - dir.angle_between(Vec2::Y))
        };

        for (v0, v1) in arrow {
            let v0 = transform(v0);
            let v1 = transform(v1);

            line(v0.x, v0.y, v1.x, v1.y, color);
        }
    }
}

enum LocalizatorTy {
    Player,
    Police,
    Contract,
}

static mut INDICATORS: Vec<Localizator> = Vec::new();

pub fn tic() {
    let indicators = unsafe { INDICATORS.drain(..) };

    for indicator in indicators {
        indicator.draw_ex();
    }
}
