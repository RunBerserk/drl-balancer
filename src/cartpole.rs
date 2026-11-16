//! Classic CartPole dynamics (OpenAI Gym / Barto–Sutton–Anderson).

/// Discrete push on the cart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Left,
    Right,
}

/// Full observable state.
#[derive(Clone, Copy, Debug)]
pub struct CartPole {
    pub x: f32,
    pub x_dot: f32,
    /// Angle from upright (radians); 0 = vertical.
    pub theta: f32,
    pub theta_dot: f32,
}

pub const RAIL_HALF: f32 = 2.4;
pub const POLE_LEN: f32 = 1.0;
pub const CART_W: f32 = 0.5;
pub const CART_H: f32 = 0.3;

/// Fixed integration step (Gym default).
pub const TAU: f32 = 0.02;

/// Observation size: `x, x_dot, theta, theta_dot`.
pub const OBS_DIM: usize = 4;

/// Discrete actions: Left / Right.
pub const N_ACTIONS: usize = 2;

/// Gym CartPole-v1 episode length cap.
pub const MAX_STEPS: usize = 500;

const GRAVITY: f32 = 9.8;
const MASS_CART: f32 = 1.0;
const MASS_POLE: f32 = 0.1;
const TOTAL_MASS: f32 = MASS_CART + MASS_POLE;
/// Distance from pivot to pole center of mass (half pole length).
const LENGTH: f32 = 0.5;
const POLEMASS_LENGTH: f32 = MASS_POLE * LENGTH;
const FORCE_MAG: f32 = 10.0;
/// Fail angle (±12°), Gym CartPole-v1.
const THETA_THRESHOLD: f32 = 12.0_f32.to_radians();
/// Uniform-rod inertia about the pivot: `I = (1/3) m (2ℓ)² = (4/3) m ℓ²`.
/// Appears in the θ̈ denominator as the factor `4/3` (Barto–Sutton–Anderson / Gym).
const POLE_INERTIA_FACTOR: f32 = 4.0 / 3.0;
const RESET_NOISE: f32 = 0.05;

#[derive(Clone, Copy, Debug)]
pub struct StepResult {
    pub reward: f32,
    pub done: bool,
}

impl CartPole {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            x_dot: 0.0,
            theta: 0.0,
            theta_dot: 0.0,
        }
    }

    pub fn reset_to(&mut self, x: f32, x_dot: f32, theta: f32, theta_dot: f32) {
        self.x = x;
        self.x_dot = x_dot;
        self.theta = theta;
        self.theta_dot = theta_dot;
    }

    /// Small uniform noise in `±`[`RESET_NOISE`] (Gym-style).
    pub fn reset_random(&mut self, rng: &mut impl rand::Rng) {
        let mut noise = || rng.random_range(-RESET_NOISE..RESET_NOISE);
        self.reset_to(noise(), noise(), noise(), noise());
    }

    pub fn observation(&self) -> [f32; OBS_DIM] {
        [self.x, self.x_dot, self.theta, self.theta_dot]
    }

    pub fn is_done(&self) -> bool {
        self.x < -RAIL_HALF
            || self.x > RAIL_HALF
            || self.theta < -THETA_THRESHOLD
            || self.theta > THETA_THRESHOLD
    }

    /// One physics step with discrete action (±[`FORCE_MAG`]).
    pub fn step(&mut self, action: Action) -> StepResult {
        let force = match action {
            Action::Left => -FORCE_MAG,
            Action::Right => FORCE_MAG,
        };
        self.step_force(force)
    }

    /// One physics step with arbitrary horizontal force (N). Use `0.0` for free fall.
    pub fn step_force(&mut self, force: f32) -> StepResult {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        // Coupled cart–pole equations (Gym / Barto–Sutton–Anderson).
        let temp =
            (force + POLEMASS_LENGTH * self.theta_dot.powi(2) * sin_theta) / TOTAL_MASS;
        let pole_mass_ratio = MASS_POLE * cos_theta.powi(2) / TOTAL_MASS;
        let theta_acc = (GRAVITY * sin_theta - cos_theta * temp)
            / (LENGTH * (POLE_INERTIA_FACTOR - pole_mass_ratio));
        let x_acc = temp - POLEMASS_LENGTH * theta_acc * cos_theta / TOTAL_MASS;

        self.x += TAU * self.x_dot;
        self.x_dot += TAU * x_acc;
        self.theta += TAU * self.theta_dot;
        self.theta_dot += TAU * theta_acc;

        let done = self.is_done();
        StepResult {
            reward: if done { 0.0 } else { 1.0 },
            done,
        }
    }
}

impl Default for CartPole {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upright_at_rest_stays_near_zero_without_force() {
        let mut env = CartPole::new();
        for _ in 0..10 {
            env.step_force(0.0);
        }
        assert!(env.x.abs() < 1e-5);
        assert!(env.theta.abs() < 1e-5);
    }

    #[test]
    fn small_angle_falls_without_force() {
        let mut env = CartPole::new();
        env.theta = 0.05;
        for _ in 0..50 {
            env.step_force(0.0);
        }
        assert!(env.theta.abs() > 0.05);
    }

    #[test]
    fn out_of_bounds_marks_done() {
        let mut env = CartPole::new();
        env.x = RAIL_HALF + 0.01;
        assert!(env.is_done());
        let result = env.step(Action::Left);
        assert!(result.done);
        assert_eq!(result.reward, 0.0);
    }
}
