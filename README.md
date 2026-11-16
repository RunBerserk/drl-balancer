# DRL Balancer

Testprojekt für Deep Reinforcement Learning mit [Burn](https://burn.dev/) in Rust. Ziel: einen Agenten trainieren, der den klassischen **CartPole**-Balancierstab aufrecht hält.

## Idee

CartPole ist eine einfache Physiksimulation: Ein Wagen fährt auf einer Schiene, darauf steht ein Stab. Der Agent steuert den Wagen nach links oder rechts und soll den Stab möglichst lange im Gleichgewicht halten.

## Stack

| Komponente | Technologie |
|------------|-------------|
| Sprache | Rust |
| Demo / Render | [macroquad](https://github.com/not-fl3/macroquad) |
| ML / Autodiff | [Burn](https://burn.dev/) 0.21 (`ndarray` + autodiff) |
| Umgebung | CartPole (Gym-Dynamik) |
| Algorithmus | REINFORCE (Policy Gradient) |

## Schnellstart

### Manuelle Demo

```bash
cargo run
```

**A/D** / **Pfeiltasten** schubsen den Wagen; **R** (oder **Space** nach Fail) reset.

### Headless Training

```bash
cargo run --release --bin train
```

Trainiert eine Policy mit REINFORCE und speichert nach `artifacts/policy.mpk` (wenn Moving-Average-Reward ≥ 475 oder nach max. Episoden).

```bash
cargo test
```

Kanban: [docs/status/status.md](docs/status/status.md)

## Status

CartPole-Env, macroquad-Demo, Burn-REINFORCE-Training. Als Nächstes: trainierte Policy in der Demo evaluieren.

## Lizenz

Noch offen.
