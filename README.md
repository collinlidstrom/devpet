# DevPet

DevPet is an original developer-themed virtual-pet game built PC-first as a deterministic reference model for a future Analogue Pocket openFPGA core.

## Current status

**Playable personality milestone.** The PC build now covers the core MVP loop:

- Egg hatches into Byte after deterministic simulated time.
- Ten original, code-defined 32 x 32 Byte concepts using a four-index palette.
- Canonical Orb Byte + Core renderer with animated Core state feedback.
- In-game Byte Concepts gallery for reviewing every candidate sprite.
- Care actions: food, coffee, sleep, and medicine.
- Code projects: fix bug, build feature, refactor, and ship release.
- Bug Squash, a 20-second mouse minigame.
- Health, energy, happiness, focus, fullness, XP, levels, projects, bugs, and care history.
- Personality derived from how Byte is raised.
- Deterministic evolution from Byte into **Bot**, **Beast**, or **Ghost**.
- Profile and Evolution screens.
- Versioned local JSON saves, autosave, atomic replacement, and capped offline progression.
- Framework-independent integer-only simulation with unit tests.
- GitHub Actions format, lint, and test checks.
- Dedicated sprite validation workflow with a downloadable labeled contact sheet.

Run the current PC build:

```bash
cargo run -p devpet-pc
```

Controls: arrows/WASD navigate, Enter/Z confirms, Escape/X goes back, mouse plays Bug Squash, and Q saves and quits.

## Project layout

```text
devpet/
├── Cargo.toml
├── crates/
│   ├── devpet-core/
│   │   └── src/lib.rs        # authoritative deterministic game simulation
│   └── devpet-pc/
│       └── src/
│           ├── bin/
│           │   └── sprite-sheet.rs # deterministic SVG contact-sheet generator
│           ├── lib.rs        # desktop support library entry point
│           ├── main.rs       # Macroquad host, screens, input and pixel renderer
│           ├── sprites.rs    # ten validated 32 x 32 indexed Byte concepts
│           └── storage.rs    # versioned saves and offline catch-up
├── docs/
│   ├── MVP_PLAN.md           # product, architecture and milestone plan
│   └── SPRITES.md            # sprite catalog, rules and contributor workflow
└── .github/workflows/
    ├── ci.yml                # workspace fmt, clippy and tests
    └── sprite-validation.yml # art checks and generated review artifact
```

The simulation crate has no window, filesystem, audio, or wall-clock dependencies. That boundary is intentional so its state transitions can later be reproduced in hardware.

## Character system

Byte begins as an Egg and hatches into the canonical **Orb Byte + Core** form. Care history produces one deterministic first-generation evolution:

- **Bot** — stronger emphasis on coding, projects, discipline, and consistent care.
- **Beast** — stronger emphasis on play, feeding, and active interaction.
- **Ghost** — emerges after repeated care mistakes and represents adaptation through adversity.

The Profile screen exposes Byte's current personality while the Evolution screen keeps future branches obscured until they occur.

The Profile screen also opens a **Byte Concepts** gallery containing Orb, Pixel, Terminal, Bit, CRT, Bug, Slime, Bot, Ghost, and Core Byte. Each design is stored as an exact 32 x 32 indexed bitmap generated at compile time. See [docs/SPRITES.md](docs/SPRITES.md) for the catalog and validation contract.

## Near-term plans

1. Expand the canonical Orb Byte artwork into a complete animation sheet: Idle, Blink, Walk, Happy, Sad, Sleep, Eat, Coffee, Code, Error, Play, and Evolution.
2. Add a proper hatch/evolution transition sequence and stronger Core-state animation.
3. Complete System, Devlog, and Inventory screens.
4. Add original sound effects, audio settings, gamepad verification, and reset confirmation.
5. Harden persistence with migration/corruption tests and explicit seven-day hibernation presentation.
6. Add golden state-transition vectors and fast-clock/debug tooling.
7. Tune care/evolution thresholds through PC playtesting.
8. Package and smoke-test the Windows v0.1.0 release.

## Future Pocket plan

After the PC rules are playtested and frozen, export golden vectors containing initial state, timed inputs, and expected snapshots. Reimplement the simulation/renderer state machines in SystemVerilog, replay the vectors in an RTL testbench, then add the Analogue Pocket platform wrapper and openFPGA packaging.

See [docs/MVP_PLAN.md](docs/MVP_PLAN.md) for the detailed roadmap and acceptance criteria.

## Principles

- Original characters, art, sound, names, and rules
- Small, readable deterministic systems
- Integer-only authoritative gameplay
- Clear separation between simulation and platform code
- Hardware-aware constraints from the first prototype
- Playable milestones before content expansion

## Technical references

- [agg23/fpga-tamagotchi](https://github.com/agg23/fpga-tamagotchi) — FPGA project/package architecture reference
- [Analogue Pocket Guide](https://github.com/latentDaniel/analogue-pocket-guide) — openFPGA ecosystem reference

DevPet is an independent project and is not affiliated with or endorsed by Bandai or Analogue.
