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
