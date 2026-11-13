# DRL Balancer

Testprojekt für Deep Reinforcement Learning mit [Burn](https://burn.dev/) in Rust. Ziel: einen Agenten trainieren, der den klassischen **CartPole**-Balancierstab aufrecht hält.

## Idee

CartPole ist eine einfache Physiksimulation: Ein Wagen fährt auf einer Schiene, darauf steht ein Stab. Der Agent steuert den Wagen nach links oder rechts und soll den Stab möglichst lange im Gleichgewicht halten.

Dieses Repo dient als Lern- und Experimentierumgebung für:

- Burn (Tensor-Framework in Rust)
- Policy-Gradient- / Actor-Critic-Ansätze (z. B. REINFORCE, PPO)
- Training-Loops, Rewards und Episode-Metriken

## Stack (geplant)

| Komponente | Technologie |
|------------|-------------|
| Sprache | Rust |
| ML / Autodiff | [Burn](https://burn.dev/) |
| Umgebung | CartPole (eigene Simulation oder Gym-ähnlich) |
| Backend | z. B. `NdArray` (CPU) oder `Wgpu` / `Cuda` |

## Schnellstart

```bash
cargo run
```

Erwartete Ausgabe: `Hello, world!`

## Status

Frühphase — Cargo-Helloworld und Git-Grundgerüst. Training und CartPole-Umgebung folgen.

## Nächste Schritte

1. Burn-Dependencies ergänzen
2. CartPole-Umgebung (State, Action, Reward, Done) implementieren
3. Einfaches Policy-Netzwerk in Burn bauen
4. Training-Loop und Logging
5. Optional: Visualisierung / Evaluation nach dem Training

## Lizenz

Noch offen.
