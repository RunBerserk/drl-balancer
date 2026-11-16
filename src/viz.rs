use macroquad::prelude::*;

use drl_balancer::cartpole::{CartPole, CART_H, CART_W, POLE_LEN, RAIL_HALF};

const PIXELS_PER_UNIT: f32 = 120.0;

pub fn draw(
    cart: &CartPole,
    steps_alive: u32,
    episode_reward: f32,
    agent_mode: bool,
    policy_loaded: bool,
) {
    clear_background(Color::from_rgba(24, 28, 36, 255));
    draw_scene(cart);
    draw_hud(cart, steps_alive, episode_reward, agent_mode, policy_loaded);
}

fn world_to_screen(wx: f32, wy: f32) -> (f32, f32) {
    let cx = screen_width() * 0.5;
    let cy = screen_height() * 0.62;
    (cx + wx * PIXELS_PER_UNIT, cy - wy * PIXELS_PER_UNIT)
}

fn draw_scene(cart: &CartPole) {
    let (x0, y0) = world_to_screen(-RAIL_HALF - 0.2, 0.0);
    let (x1, y1) = world_to_screen(RAIL_HALF + 0.2, 0.0);
    draw_line(x0, y0, x1, y1, 4.0, Color::from_rgba(90, 100, 120, 255));

    let (cx, cy) = world_to_screen(cart.x, CART_H * 0.5);
    let pw = CART_W * PIXELS_PER_UNIT;
    let ph = CART_H * PIXELS_PER_UNIT;
    draw_rectangle(
        cx - pw * 0.5,
        cy - ph * 0.5,
        pw,
        ph,
        Color::from_rgba(70, 140, 220, 255),
    );

    let wheel_r = 0.08 * PIXELS_PER_UNIT;
    for side in [-0.18_f32, 0.18] {
        let (wx, wy) = world_to_screen(cart.x + side, 0.0);
        draw_circle(wx, wy, wheel_r, Color::from_rgba(40, 44, 52, 255));
    }

    let (px, py) = world_to_screen(cart.x, CART_H);
    let tip_x = cart.x + POLE_LEN * cart.theta.sin();
    let tip_y = CART_H + POLE_LEN * cart.theta.cos();
    let (tx, ty) = world_to_screen(tip_x, tip_y);
    draw_line(px, py, tx, ty, 6.0, Color::from_rgba(220, 120, 70, 255));
    draw_circle(px, py, 5.0, Color::from_rgba(230, 230, 235, 255));
    draw_circle(tx, ty, 7.0, Color::from_rgba(240, 200, 80, 255));
}

fn draw_hud(
    cart: &CartPole,
    steps_alive: u32,
    episode_reward: f32,
    agent_mode: bool,
    policy_loaded: bool,
) {
    let mode = if !policy_loaded {
        "manual (no policy.mpk)"
    } else if agent_mode {
        "agent"
    } else {
        "manual"
    };

    let hint = if cart.is_done() {
        "Fallen / off rail — R or Space to reset"
    } else if agent_mode {
        "Agent driving  |  hold A/D to override  |  P manual  |  R reset"
    } else {
        "A/D or arrows — push  |  P agent  |  R reset"
    };

    draw_text(
        format!("mode: {mode}"),
        16.0,
        28.0,
        22.0,
        Color::from_rgba(200, 205, 215, 255),
    );
    draw_text(hint, 16.0, 54.0, 18.0, Color::from_rgba(160, 170, 185, 255));
    draw_text(
        format!(
            "steps {steps_alive}  reward {episode_reward:.0}   x={:+.2}  dx={:+.2}  th={:+.2}  dth={:+.2}",
            cart.x, cart.x_dot, cart.theta, cart.theta_dot
        ),
        16.0,
        78.0,
        18.0,
        Color::from_rgba(160, 170, 185, 255),
    );
}
