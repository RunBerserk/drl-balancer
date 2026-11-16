//! Load a trained policy and pick actions for the live demo.

use burn::{
    backend::NdArray,
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::backend::BackendTypes,
};

use crate::cartpole::Action;
use crate::policy::{PolicyNet, POLICY_ARTIFACT, POLICY_HIDDEN};

type InferB = NdArray;
type Device = <NdArray as BackendTypes>::Device;

pub struct Agent {
    policy: PolicyNet<InferB>,
    device: Device,
}

impl Agent {
    /// Load `artifacts/policy.mpk` if present and readable.
    pub fn try_load() -> Option<Self> {
        let device = Default::default();
        let recorder = CompactRecorder::new();
        let record = recorder.load(POLICY_ARTIFACT.into(), &device).ok()?;
        let policy = PolicyNet::<InferB>::new(POLICY_HIDDEN, &device).load_record(record);
        Some(Self { policy, device })
    }

    /// Greedy action (argmax logits).
    pub fn act(&self, observation: [f32; 4]) -> Action {
        let obs = Tensor::<InferB, 1>::from_floats(observation, &self.device).unsqueeze::<2>();
        let logits = self.policy.forward(obs);
        let values = logits.to_data().to_vec::<f32>().expect("logits");
        if values[0] >= values[1] {
            Action::Left
        } else {
            Action::Right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_and_act_if_artifact_present() {
        let Some(agent) = Agent::try_load() else {
            return;
        };
        let action = agent.act([0.0, 0.0, 0.02, 0.0]);
        assert!(matches!(action, Action::Left | Action::Right));
    }
}
