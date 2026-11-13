# Project Status

Last updated: 2026-11-13

**Time invested:** ~0h _(Mensch — Agent nicht anfassen)_

<!-- AGENT-README:
  In Progress = Mensch arbeitet hier per Hand. NICHT editieren, nicht als Aufgabe nehmen.
  Next Up     = geordnete Warteschlange. Oberster Eintrag = nächster Agent-Schritt. Von oben nach unten.
  Later       = Backlog. Nicht ohne Rückfrage starten.
  Ticket-Format: `[id] (module) Titel` — id ist stabil, für Referenzen.
  Done = done.md — status.md bleibt Kanban-Pipeline.
-->

## Current Phase

Scaffold — Hello World + Git. **Fokus:** macroquad-Demo (Schiene/Wagen/Stab), danach Physik, dann Burn/Training.

## Done

Siehe [done.md](done.md).

## In Progress _(Mensch — Agent nicht anfassen)_

_(none)_

## Next Up

- `[viz-macroquad-cartpole]` (viz) — macroquad-Fenster: Schiene, Wagen, Stab; State erst manuell/Tastatur
  - **Acceptance:** `cargo run` zeigt CartPole-Look; links/rechts bewegt den Wagen (noch ohne echte Physik ok).

## Later (Backlog)

- `[env-cartpole-physics]` (env) — State `x, ẋ, θ, θ̇` + `step(action)`; Renderer nur zeichnen
- `[rl-burn-policy]` (rl) — Burn Policy-Netz + Training-Loop (headless)
- `[demo-eval-loop]` (demo) — trainierte Policy in macroquad-Eval einhängen
- `[docs-readme-demo]` (docs) — README: Run-Demo, Train, Stack (Burn + macroquad)

## Gaps / Notes

_(noch keine — bei Bedarf gaps.md / notes.md)_
