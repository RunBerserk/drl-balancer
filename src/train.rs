//! Headless REINFORCE training on CartPole.

use burn::{
    backend::{Autodiff, NdArray},
    grad_clipping::GradientClippingConfig,
    module::AutodiffModule,
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    record::{CompactRecorder, Recorder},
    tensor::{
        activation, backend::BackendTypes, ElementConversion, Int, TensorData,
    },
};
use rand::Rng;

use crate::cartpole::{Action, CartPole, MAX_STEPS};
use crate::policy::PolicyNet;

type B = Autodiff<NdArray>;
type Device = <NdArray as BackendTypes>::Device;

const HIDDEN: usize = 64;
const LR: f64 = 1e-2;
const GAMMA: f32 = 0.99;
const EPISODES: usize = 400;
const LOG_EVERY: usize = 20;
const SOLVE_REWARD: f32 = 475.0;
const ARTIFACT: &str = "artifacts/policy";

pub fn run() {
    let device = Default::default();
    let mut policy = PolicyNet::<B>::new(HIDDEN, &device);
    let mut optim = AdamConfig::new()
        .with_grad_clipping(Some(GradientClippingConfig::Norm(0.5)))
        .init();
    let mut rng = rand::rng();

    let mut best = 0.0_f32;
    let mut window = 0.0_f32;

    println!("REINFORCE CartPole — {EPISODES} episodes, lr={LR}, hidden={HIDDEN}");

    for ep in 1..=EPISODES {
        let (reward, loss) = train_episode(&mut policy, &mut optim, &mut rng, &device);
        window = if ep == 1 {
            reward
        } else {
            0.95 * window + 0.05 * reward
        };
        best = best.max(reward);

        if ep % LOG_EVERY == 0 || reward >= SOLVE_REWARD {
            println!(
                "ep {ep:4}  reward {reward:6.1}  avg {window:6.1}  best {best:6.1}  loss {loss:.4}"
            );
        }

        if window >= SOLVE_REWARD {
            println!("solved (moving avg >= {SOLVE_REWARD}) at episode {ep}");
            break;
        }
    }

    save_policy(&policy, &device);
    println!("saved policy → {ARTIFACT}.mpk");
}

fn train_episode(
    policy: &mut PolicyNet<B>,
    optim: &mut impl Optimizer<PolicyNet<B>, B>,
    rng: &mut impl Rng,
    device: &Device,
) -> (f32, f32) {
    let mut env = CartPole::new();
    env.reset_random(rng);

    let mut log_probs: Vec<Tensor<B, 1>> = Vec::new();
    let mut rewards: Vec<f32> = Vec::new();

    for _ in 0..MAX_STEPS {
        let obs = Tensor::<B, 1>::from_floats(env.observation(), device).unsqueeze::<2>();
        let logits = policy.forward(obs);
        let log_sm = activation::log_softmax(logits.clone(), 1);
        let probs = activation::softmax(logits, 1);

        let action_idx = sample_action(&probs, rng);
        let indices = Tensor::<B, 1, Int>::from_ints([action_idx as i32], device);
        let log_prob = log_sm.select(1, indices).reshape([1]);
        log_probs.push(log_prob);

        let action = if action_idx == 0 {
            Action::Left
        } else {
            Action::Right
        };
        let step = env.step(action);
        rewards.push(step.reward);
        if step.done {
            break;
        }
    }

    let returns = discounted_returns(&rewards, GAMMA);
    let total_reward: f32 = rewards.iter().sum();

    let log_probs = Tensor::cat(log_probs, 0);
    let returns_t = Tensor::<B, 1>::from_data(TensorData::new(returns, [rewards.len()]), device);
    // Advantage: normalize returns for lower variance.
    let mean = returns_t.clone().mean();
    let std = returns_t.clone().var(0).sqrt().clamp_min(1e-8);
    let adv = returns_t.sub(mean).div(std);

    let loss = log_probs.mul(adv).mean().neg();
    let loss_val = loss.clone().into_scalar().elem::<f32>();

    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, policy);
    *policy = optim.step(LR, policy.clone(), grads);

    (total_reward, loss_val)
}

fn sample_action<BB: Backend>(probs: &Tensor<BB, 2>, rng: &mut impl Rng) -> usize {
    let data = probs.to_data();
    let values = data.to_vec::<f32>().expect("prob floats");
    let p0 = values[0];
    if rng.random::<f32>() < p0 {
        0
    } else {
        1
    }
}

fn discounted_returns(rewards: &[f32], gamma: f32) -> Vec<f32> {
    let mut out = vec![0.0; rewards.len()];
    let mut running = 0.0_f32;
    for (i, r) in rewards.iter().enumerate().rev() {
        running = r + gamma * running;
        out[i] = running;
    }
    out
}

fn save_policy(policy: &PolicyNet<B>, _device: &Device) {
    let _ = std::fs::create_dir_all("artifacts");
    let recorder = CompactRecorder::new();
    // Save inner (non-autodiff) weights for later inference.
    let inner = policy.valid();
    recorder
        .record(inner.into_record(), ARTIFACT.into())
        .expect("save policy");
}
