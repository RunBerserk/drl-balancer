use macroquad::prelude::*;

use drl_balancer::agent::Agent;
use drl_balancer::cartpole::{CartPole, MAX_STEPS, TAU};
use crate::input;
use crate::viz;

pub async fn run() {
    let agent = Agent::try_load();
    let mut agent_mode = agent.is_some();
    if agent.is_some() {
        println!("loaded policy — agent mode (P toggles manual)");
    } else {
        println!("no artifacts/policy.mpk — manual mode (train with: cargo run --release --bin train)");
    }

    let mut env = CartPole::new();
    reset_episode(&mut env);
    let mut accum = 0.0_f32;
    let mut steps_alive = 0_u32;
    let mut episode_reward = 0.0_f32;
    let mut done_timer = 0.0_f32;

    loop {
        if input::toggle_agent_pressed() && agent.is_some() {
            agent_mode = !agent_mode;
        }

        let want_reset = input::reset_pressed()
            || (env.is_done() && is_key_pressed(KeyCode::Space))
            || (agent_mode && env.is_done() && done_timer > 1.0)
            || steps_alive >= MAX_STEPS as u32;

        if want_reset {
            reset_episode(&mut env);
            steps_alive = 0;
            episode_reward = 0.0;
            done_timer = 0.0;
            accum = 0.0;
        }

        if env.is_done() {
            done_timer += get_frame_time();
        }

        accum += get_frame_time();
        while accum >= TAU {
            accum -= TAU;
            if env.is_done() || steps_alive >= MAX_STEPS as u32 {
                break;
            }

            let result = if agent_mode {
                if let Some(manual) = input::action() {
                    env.step(manual)
                } else if let Some(agent) = agent.as_ref() {
                    env.step(agent.act(env.observation()))
                } else {
                    env.step_force(0.0)
                }
            } else {
                match input::action() {
                    Some(action) => env.step(action),
                    None => env.step_force(0.0),
                }
            };

            episode_reward += result.reward;
            if !result.done {
                steps_alive += 1;
            }
        }

        viz::draw(
            &env,
            steps_alive,
            episode_reward,
            agent_mode,
            agent.is_some(),
        );
        next_frame().await;
    }
}

fn reset_episode(env: &mut CartPole) {
    let noise = || macroquad::rand::gen_range(-0.05_f32, 0.05);
    env.reset_to(noise(), noise(), noise(), noise());
}
