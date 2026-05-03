---
# project-os-npym
title: Hide Webamp equalizer window
status: completed
type: feature
priority: normal
created_at: 2026-04-16T22:59:27Z
updated_at: 2026-04-16T23:02:53Z
---

The equalizer window in the Webamp fixture is visual clutter — hide it on init so only the main player and playlist remain.

## Summary of Changes

- src/components/webamp.rs:
  - Renamed serde field `initialWindowLayout` → `windowLayout` to match Webamp's actual public API. The wrong key was silently ignored, so prior positioning code never took effect.
  - Renamed `WebampWindowPosition` → `WebampWindowSlot` and added a `closed: bool` field (skipped on serialize when false).
  - Equalizer now passed with `closed: true` and a placeholder position.
  - Playlist offset adjusted from `top + 232` → `top + 116` so it sits directly under main now that equalizer is hidden.

cargo check + clippy pass on wasm32-unknown-unknown.
