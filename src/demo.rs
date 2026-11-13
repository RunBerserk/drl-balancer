use macroquad::prelude::*;

use crate::cartpole::CartPole;
use crate::input;
use crate::viz;

pub async fn run() {
    let mut cart = CartPole::new();

    loop {
        let dt = get_frame_time();
        cart.steer(input::steer(), dt);
        viz::draw(&cart);
        next_frame().await;
    }
}
