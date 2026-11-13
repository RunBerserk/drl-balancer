use macroquad::prelude::*;

/// Keyboard steer in `[-1, 1]` (A/D or arrow keys).
pub fn steer() -> f32 {
    let mut steer = 0.0_f32;
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        steer -= 1.0;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        steer += 1.0;
    }
    steer
}
