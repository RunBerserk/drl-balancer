use macroquad::prelude::*;

use drl_balancer::cartpole::Action;

/// Keyboard action: Left / Right, or `None` if no key held.
pub fn action() -> Option<Action> {
    let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
    let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
    match (left, right) {
        (true, false) => Some(Action::Left),
        (false, true) => Some(Action::Right),
        _ => None,
    }
}

pub fn reset_pressed() -> bool {
    is_key_pressed(KeyCode::R)
}
