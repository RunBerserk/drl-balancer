# Done

## Scaffold: Rust + Git

- [x] 1. Cargo binary `drl-balancer` + Hello World
- [x] 2. `.gitignore`, README
- [x] 3. GitHub `main` (`RunBerserk/drl-balancer`)

## Viz: macroquad CartPole `[viz-macroquad-cartpole]`

- [x] 1. Dependency `macroquad` 0.4
- [x] 2. Window: rail, cart, wheels, pole, pivot
- [x] 3. A/D / arrows move the cart; soft lean (no physics)
- [x] 4. README quickstart updated

## Env: CartPole physics `[env-cartpole-physics]`

- [x] 1. State `x, x_dot, theta, theta_dot` + `Action` + `step` / `step_force`
- [x] 2. Gym params (τ=0.02, force ±10, fail if |x|>2.4 or |θ|>12°)
- [x] 3. Demo: fixed timestep, keyboard → force, reset R/Space
- [x] 4. Unit tests; viz is render-only

## RL: Burn policy + training `[rl-burn-policy]`

- [x] 1. Lib layout: `cartpole` / `policy` / `train`; `train` binary
- [x] 2. Burn 0.21 policy MLP (4→64→2) on `Autodiff<NdArray>`
- [x] 3. REINFORCE loop (Adam, return norm, early-stop avg≥475)
- [x] 4. Saves `artifacts/policy.mpk`; README training section

## Demo: policy eval `[demo-eval-loop]`

- [x] 1. `agent::Agent` loads `artifacts/policy.mpk` (NdArray inference)
- [x] 2. Demo starts in agent mode when weights exist; **P** toggles manual
- [x] 3. Greedy actions; keyboard override; auto-reset after fail / 500 steps
- [x] 4. HUD mode indicator; load test when artifact is present

## Docs: README demo `[docs-readme-demo]`

- [x] 1. Stack table (Burn + macroquad + env + REINFORCE)
- [x] 2. Run-demo / run-train / tests including keys and `policy.mpk`
- [x] 3. Binary overview, layout, requirements
