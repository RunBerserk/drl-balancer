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

### Training

```bash
cargo run --release --bin train
```

Speichert die Policy nach `artifacts/policy.mpk`.

### Demo (Agent oder manuell)

```bash
cargo run
```

Wenn `artifacts/policy.mpk` existiert, startet die Demo im **Agent-Modus** (greedy).

| Taste | Aktion |
|-------|--------|
| **P** | Agent ↔ manuell umschalten |
| **A/D** / Pfeile | Schub (manuell, oder Override im Agent-Modus) |
| **R** / **Space** | Reset |

Ohne Policy-Datei: nur manuell (Hinweis in der Konsole).

```bash
cargo test
```

Kanban: [docs/status/status.md](docs/status/status.md)

## Status

Env + REINFORCE-Training + Live-Eval in macroquad.

## Lizenz

Noch offen.
