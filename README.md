# DevPet

DevPet is an original virtual-pet game being built PC-first as the deterministic reference model for a future Analogue Pocket openFPGA core.

The first release will let a player hatch and care for Byte, play a short minigame, persist progress across sessions, and reach one of three care-driven evolutions.

## Status

**M0/M1 implementation in progress.** The PC prototype now has the deterministic core, classic virtual-pet home UI, and the original 32x32 Orb Byte + Core character direction.

## Technical direction

- Rust workspace
- Framework-independent, integer-only simulation core
- Macroquad desktop host
- 160 x 144 logical pixel canvas
- Versioned local saves and deterministic offline progression
- Golden test vectors for a later SystemVerilog implementation

## Roadmap

1. Build and validate the PC MVP.
2. Freeze the gameplay rules and generate golden state-transition vectors.
3. Port the simulation and renderer to SystemVerilog.
4. Package and test the core on Analogue Pocket hardware.

See the [complete PC MVP plan](docs/MVP_PLAN.md) for scope, architecture, milestones, acceptance tests, and the implementation backlog.

## Project principles

- Original characters, art, sound, names, and rules
- Small, readable, deterministic systems
- Clear separation between gameplay and platform code
- Hardware-aware constraints from the first prototype
- Playable milestones before content expansion

## Inspiration and references

- [agg23/fpga-tamagotchi](https://github.com/agg23/fpga-tamagotchi) — technical reference for FPGA project and Pocket packaging structure
- [Analogue Pocket Guide](https://github.com/latentDaniel/analogue-pocket-guide) — openFPGA ecosystem guide

DevPet is an independent project and is not affiliated with or endorsed by Bandai or Analogue.
