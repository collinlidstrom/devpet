# UI previews

The **UI Preview** GitHub Actions workflow builds and runs the native desktop
renderer under Xvfb with Mesa software rendering. It runs on pull requests,
pushes to `main`, and manually through **Actions → UI Preview → Run workflow**
(the manual option becomes available once the workflow is merged into `main`).

Open a completed run and download the `devpet-ui-preview-<run-id>` artifact
from its summary or Artifacts section. Extract the ZIP and open `index.html`
in a browser. It contains Home, Care, Code, Play, Bug Squash, Profile,
Evolution, and all ten Byte concept screens, plus their individual PNGs.
Artifacts are retained for 14 days and require GitHub sign-in to download.

These are static UI previews, not a playable browser build or an interaction
test. The app renders a fixed, hatched Byte at age 30 minutes, freezes animation
and the minigame timer, ignores input, and never loads or writes a save in this
mode. Each case renders twice before capture. The normal game remains unchanged
when `DEVPET_UI_PREVIEW_DIR` is unset.

## Generate locally (Linux)

Install `xvfb`, `xauth`, `libgl1-mesa-dri`, `libgl1`, `libx11-6`, and `libxi6`, then:

```sh
cargo build --locked -p devpet-pc --bin devpet-pc
LIBGL_ALWAYS_SOFTWARE=1 DEVPET_UI_PREVIEW_DIR=ui-preview \
  timeout 90s xvfb-run -a -s '-screen 0 640x576x24' target/debug/devpet-pc
python3 scripts/ui_preview_gallery.py ui-preview
```

The gallery builder rejects missing captures and incorrect image dimensions.
When adding screens or sprite concepts, update the capture cases in
`crates/devpet-pc/src/main.rs` and the expected names in
`scripts/ui_preview_gallery.py` together. Use an empty output directory when
changing the capture set. Driver and font rendering differences mean these
captures should not be treated as cross-platform pixel-perfect baselines.
