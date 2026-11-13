/// Visual-only cart state (no physics yet).
#[derive(Clone, Copy, Debug)]
pub struct CartPole {
    /// Cart position along the rail (world units, 0 = center).
    pub x: f32,
    /// Pole angle from upright (radians); soft lean while steering.
    pub theta: f32,
}

pub const RAIL_HALF: f32 = 2.4;
pub const CART_SPEED: f32 = 2.5;
pub const POLE_LEN: f32 = 1.0;
pub const CART_W: f32 = 0.5;
pub const CART_H: f32 = 0.3;

impl CartPole {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            theta: 0.0,
        }
    }

    /// Manual demo step: steer in `[-1, 1]`, no real dynamics.
    pub fn steer(&mut self, steer: f32, dt: f32) {
        self.x = (self.x + steer * CART_SPEED * dt).clamp(-RAIL_HALF, RAIL_HALF);
        let lean_target = -steer * 0.25;
        self.theta += (lean_target - self.theta) * (8.0 * dt).min(1.0);
    }
}
