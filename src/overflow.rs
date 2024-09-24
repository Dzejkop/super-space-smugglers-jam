use crate::prelude::*;

pub struct OverflowIndicator {
    at: Vec2,
    id: OverflowIndicatorTy,
}

impl OverflowIndicator {
    pub fn player(at: Vec2) -> Self {
        Self {
            at,
            id: OverflowIndicatorTy::Player,
        }
    }

    pub fn police(at: Vec2) -> Self {
        Self {
            at,
            id: OverflowIndicatorTy::Police,
        }
    }

    pub fn draw(self) {
        unsafe {
            INDICATORS.push(self);
        }
    }

    fn draw_ex(self) {
        let Self { id, at } = self;

        let color = match id {
            OverflowIndicatorTy::Player => {
                if blink() {
                    return;
                } else {
                    5
                }
            }

            OverflowIndicatorTy::Police => {
                if blink() {
                    2
                } else {
                    10
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

enum OverflowIndicatorTy {
    Player,
    Police,
}

static mut INDICATORS: Vec<OverflowIndicator> = Vec::new();

pub fn tic() {
    let indicators = unsafe { INDICATORS.drain(..) };

    for indicator in indicators {
        indicator.draw_ex();
    }
}
