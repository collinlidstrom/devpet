# PC controls pass

This pass builds on the merged personality milestone and ten-sprite catalog. The earlier unpublished M0 scaffold is preserved locally, not applied over the newer implementation.

## Implemented

- Clickable Home, Care, Code, Play, and Profile menus.
- Clickable top-left headings for Back on non-game screens.
- Clickable previous/next arrows in Byte Concepts.
- Hover outline independent of keyboard focus.
- Space confirms; 1–4 opens Home actions directly.
- Shared logical pointer conversion for menus and Bug Squash.
- Letterbox clicks are rejected; the background no longer clears the entire window to green.
- Back input is consumed before the new screen handles actions, avoiding accidental activation.

## Manual Windows smoke test

1. Run `cargo run -p devpet-pc`. Click each Home action, then its top-left heading to return.
2. Click each Care item and Code project; verify the corresponding stats change.
3. Click Menu, Evolution, and the top-left heading; verify return to Profile.
4. Open Byte Concepts and click both side arrows. Check wraparound at forms 1 and 10.
5. Start Bug Squash by mouse; hit bugs, let the timer expire, and verify return to Home.
6. Leave the pointer stationary on one menu item while using arrows and Enter to select another. Keyboard selection must win.
7. Test Home shortcuts 1–4, Space to confirm, and Escape/X to return.
8. Resize to several sizes above 160×144, including non-multiples. Clicks must remain aligned; border clicks must do nothing.
9. Press Back and Confirm together: only Back should take effect.
10. Q saves and quits as before.

Automated tests cover scale mapping, letterbox rejection, button centers, and non-overlapping row boundaries. They do not substitute for running the interactive Windows smoke test.

## Not changed

Simulation timing, save handling, personality, sprite artwork, and rewards are unchanged. The existing 1-real-second to 1-pet-minute accelerated clock remains; normal-time/debug-clock separation and save hardening are still future work. Extremely small windows below the logical canvas still crop at 1× and are not a supported layout.
