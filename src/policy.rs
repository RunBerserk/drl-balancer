//! Discrete policy network for CartPole.

use burn::{
    nn::{Linear, LinearConfig, Relu},
    prelude::*,
};

use crate::cartpole::{N_ACTIONS, OBS_DIM};

/// Hidden width used by training and eval (must match saved weights).
pub const POLICY_HIDDEN: usize = 64;

/// Path stem for CompactRecorder (`artifacts/policy.mpk`).
pub const POLICY_ARTIFACT: &str = "artifacts/policy";

#[derive(Module, Debug)]
pub struct PolicyNet<B: Backend> {
    fc1: Linear<B>,
    act: Relu,
    fc2: Linear<B>,
}

impl<B: Backend> PolicyNet<B> {
    pub fn new(hidden: usize, device: &B::Device) -> Self {
        Self {
            fc1: LinearConfig::new(OBS_DIM, hidden).init(device),
            act: Relu::new(),
            fc2: LinearConfig::new(hidden, N_ACTIONS).init(device),
        }
    }

    /// Logits over actions. Input shape `[batch, 4]`, output `[batch, 2]`.
    pub fn forward(&self, obs: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.act.forward(self.fc1.forward(obs));
        self.fc2.forward(x)
    }
}
