use macroquad::prelude::*;

use crate::cartpole::{CartPole, TAU};
use crate::input;
use crate::viz;

pub async fn run() {
    let mut env = CartPole::new();
    // Tiny initial lean so the pole starts moving without a push.
    env.theta = 0.02;
    let mut accum = 0.0_f32;
    let mut steps_alive = 0_u32;
    let mut episode_reward = 0.0_f32;

    loop {
        if input::reset_pressed() || (env.is_done() && is_key_pressed(KeyCode::Space)) {
            reset_episode(&mut env);
            steps_alive = 0;
            episode_reward = 0.0;
            accum = 0.0;
        }

        accum += get_frame_time();
        while accum >= TAU {
            accum -= TAU;
            if env.is_done() {
                break;
            }
            let result = match input::action() {
                Some(action) => env.step(action),
                None => env.step_force(0.0),
            };
            episode_reward += result.reward;
            if !result.done {
                steps_alive += 1;
            }
        }

        viz::draw(&env, steps_alive, episode_reward);
        next_frame().await;
    }
}

fn reset_episode(env: &mut CartPole) {
    let noise = || macroquad::rand::gen_range(-0.05_f32, 0.05);
    env.reset_to(noise(), noise(), noise(), noise());
}
