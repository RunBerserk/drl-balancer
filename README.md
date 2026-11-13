# DRL Balancer

Testprojekt für Deep Reinforcement Learning mit [Burn](https://burn.dev/) in Rust. Ziel: einen Agenten trainieren, der den klassischen **CartPole**-Balancierstab aufrecht hält.

## Idee

CartPole ist eine einfache Physiksimulation: Ein Wagen fährt auf einer Schiene, darauf steht ein Stab. Der Agent steuert den Wagen nach links oder rechts und soll den Stab möglichst lange im Gleichgewicht halten.

Dieses Repo dient als Lern- und Experimentierumgebung für:

- Burn (Tensor-Framework in Rust)
- Policy-Gradient- / Actor-Critic-Ansätze (z. B. REINFORCE, PPO)
- Training-Loops, Rewards und Episode-Metriken

## Stack

| Komponente | Technologie |
|------------|-------------|
| Sprache | Rust |
| Demo / Render | [macroquad](https://github.com/not-fl3/macroquad) |
| ML / Autodiff | [Burn](https://burn.dev/) (geplant) |
| Umgebung | CartPole (eigene Simulation, geplant) |

## Schnellstart

```bash
cargo run
```

Öffnet ein Fenster: Schiene, Wagen, Stab. **A/D** oder **Pfeiltasten** bewegen den Wagen (noch ohne echte Physik).

Kanban: [docs/status/status.md](docs/status/status.md)

## Status

macroquad-CartPole-Demo (manuell). Als Nächstes: Physik-Env, dann Burn-Training.

## Nächste Schritte

1. CartPole-Physik (`step(action)`)
2. Burn Policy-Netz + Training
3. Policy in der Demo evaluieren

## Lizenz

Noch offen.
