# Done

## Scaffold: Rust + Git

- [x] 1. Cargo-Binary `drl-balancer` + Hello World
- [x] 2. `.gitignore`, README
- [x] 3. GitHub `main` (`RunBerserk/drl-balancer`)

## Viz: macroquad CartPole `[viz-macroquad-cartpole]`

- [x] 1. Dependency `macroquad` 0.4
- [x] 2. Fenster: Schiene, Wagen, Räder, Stab, Pivot
- [x] 3. A/D / Pfeiltasten bewegen den Wagen; weiches Lean (keine Physik)
- [x] 4. README Schnellstart angepasst

## Env: CartPole Physik `[env-cartpole-physics]`

- [x] 1. State `x, x_dot, theta, theta_dot` + `Action` + `step` / `step_force`
- [x] 2. Gym-Parameter (τ=0.02, Force ±10, Fail bei |x|>2.4 oder |θ|>12°)
- [x] 3. Demo: Fixed-Timestep, Tastatur → Force, Reset R/Space
- [x] 4. Unit-Tests; Viz nur Renderer

## RL: Burn Policy + Training `[rl-burn-policy]`

- [x] 1. Lib-Layout: `cartpole` / `policy` / `train`; Binary `train`
- [x] 2. Burn 0.21 Policy-MLP (4→64→2) auf `Autodiff<NdArray>`
- [x] 3. REINFORCE-Loop (Adam, Return-Norm, Early-Stop avg≥475)
- [x] 4. Speichert `artifacts/policy.mpk`; README Training-Abschnitt

## Demo: Policy Eval `[demo-eval-loop]`

- [x] 1. `agent::Agent` lädt `artifacts/policy.mpk` (NdArray inference)
- [x] 2. Demo startet im Agent-Modus wenn Weights da; **P** toggelt Manual
- [x] 3. Greedy-Actions; Tastatur-Override; Auto-Reset nach Fail / 500 Steps
- [x] 4. HUD Mode-Anzeige; Load-Test wenn Artifact vorhanden
