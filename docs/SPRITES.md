# DevPet Sprite Catalog

DevPet's first character-art set contains ten original Byte concepts. Every concept is an exact 32 x 32 indexed bitmap defined in Rust and rendered from the same source data in the game and the review contact sheet.

## Byte concepts

| ID | Display name | Direction |
| --- | --- | --- |
| `orb-byte` | 01 Orb Byte | Round mascot silhouette with a small state Core; the canonical Byte |
| `pixel-byte` | 02 Pixel Byte | Living square pixel with crisp digital corners |
| `terminal-byte` | 03 Terminal Byte | Terminal-shaped face with `>` and `_` prompt eyes |
| `bit-byte` | 04 Bit Byte | Oversized head and tiny virtual-pet body |
| `crt-byte` | 05 CRT Byte | Living retro monitor with screen, stand, and status light |
| `bug-byte` | 06 Bug Byte | Antennae, split wings, and software-bug silhouette |
| `slime-byte` | 07 Slime Byte | Deformable digital blob with irregular drips |
| `bot-byte` | 08 Bot Byte | Antenna, side modules, robot body, and chest Core |
| `ghost-byte` | 09 Ghost Byte | Floating body with a shifting pixel fringe |
| `core-byte` | 10 Core Byte | Compact guardian built around a large central Core |

## Sprite contract

The source of truth is `crates/devpet-pc/src/sprites.rs`.

- Canvas: exactly 32 x 32 pixels.
- Palette index `0`: transparent.
- Palette index `1`: dark outline and detail.
- Palette index `2`: body fill.
- Palette index `3`: Core/accent, recolored by state at render time.
- Every sprite must have a unique stable ID and display name.
- Visible pixels must retain at least one transparent pixel of padding on every edge.
- Canonical sprites must remain readable at native 1x scale and with nearest-neighbor integer scaling.

The compile-time `Canvas` builder emits fixed `[u8; 1024]` bitmaps. That keeps the art deterministic and makes the data straightforward to convert into a packed sprite ROM for a future hardware build.

## Reviewing the catalog

In the PC game, open `MENU`, select `BYTE CONCEPTS`, then use Left/Right or A/D to browse all ten designs.

Generate the labeled SVG contact sheet locally with:

```bash
cargo run -p devpet-pc --bin sprite-sheet -- target/byte-sprite-contact-sheet.svg
```

## Adding or changing a sprite

1. Build the bitmap with the indexed drawing primitives in `sprites.rs`.
2. Define its `Sprite32` metadata and add it to `BYTE_CONCEPTS`.
3. Add or update tests when the intentional catalog count changes.
4. Run formatting, sprite tests, linting, and the contact-sheet generator.
5. Inspect the generated sheet at native scale and enlarged scale before merging.

The `Sprite validation` GitHub Actions workflow performs these checks automatically when sprite source or tooling changes. It rejects invalid palette data, missing edge padding, duplicate IDs/names, duplicate bitmaps, formatter drift, test failures, and Clippy warnings. Each successful run uploads `byte-sprite-contact-sheet.svg` as a review artifact.
