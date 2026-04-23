---
# project-os-l810
title: Make Webamp windows non-draggable to fix Firefox RDM taskbar/Clippy drift
status: scrapped
type: feature
priority: high
created_at: 2026-04-21T16:47:21Z
updated_at: 2026-04-23T20:36:41Z
blocking:
    - project-os-s0wm
---

Remove draggable class from all Webamp title bars post-mount to prevent layout-viewport expansion

## Problem

On Firefox Desktop in Responsive Design Mode (any mobile device preset), dragging the Webamp player causes the `position: fixed` taskbar and Clippy widget to visually drift out of the viewport. Root cause: Firefox's layout viewport auto-expands during the drag (something inside Webamp's drag state widens the page), causing fixed elements to re-anchor incorrectly. A `MutationObserver` clamp on `style.left`/`style.top` (project-os-4umw) did **not** fix it — the expansion isn't solely from inline position overflow; Webamp's drag machinery causes it by some other means.

## Decision

Workaround option #3 from project-os-s0wm: make all three Webamp windows (main, playlist, equalizer) non-draggable while keeping all playback controls fully interactive.

## Approach — RECOMMENDED: Remove `draggable` class from title bars via JS after mount

**How Webamp drag works (verified in WindowManager.tsx):**

```
if (!(e.target as HTMLElement).classList.contains("draggable")) {
  return;
}
```

Dragging only initiates if the clicked element has the CSS class `"draggable"`. No Webamp constructor option disables this. The relevant elements per window:

| Window | Drag-handle element | Class |
|---|---|---|
| Main | `#title-bar` (div) | `"selected draggable"` |
| Playlist | `.playlist-top` (div) | `"playlist-top draggable"` |
| Equalizer | `.equalizer-top` (div) | `"equalizer-top title-bar draggable"` |

The child buttons (Close, Shade/Minimize, EqTitleButtons) do **not** carry the `"draggable"` class — they just dispatch Redux actions. Removing `"draggable"` from the three title-bar divs prevents any drag from starting but leaves all button clicks unaffected.

**Implementation:** After Webamp renders, run a short polling loop (e.g. `Timeout::new(200, …)`) waiting for `#title-bar` to appear in the DOM, then:

```js
for sel in ["#title-bar", ".playlist-top", ".equalizer-top"] {
    document.querySelector(sel)?.classList.remove("draggable");
}
```

In Rust/WASM: use `web_sys::Document::query_selector` to find each element and `DomTokenList::remove1` on its class list. Store nothing extra — this is a one-shot mutation, no listener to clean up.

**Why this beats the alternatives:**

- **Option 1 — Native Webamp `draggable: false` constructor option** — does not exist. Verified by reading docs and Webamp constructor source. Rejected: not available.
- **Option 2 — CSS `pointer-events: none` on title bar, re-enable on children** — fragile: requires enumerating every interactive child (Close, Shade, MiniTime, ContextMenuTarget, WinampButton…) and the list can change across Webamp versions. Rejected: brittle enumeration.
- **Option 3 (chosen above) — Remove `draggable` class post-mount** — surgical, zero side-effects on controls, no ongoing listeners. WindowManager's guard is the single authoritative gate.
- **Option 4 — `stopImmediatePropagation` on mousedown/touchstart/pointerdown on title bar** — capturing listener on title bar would fire before close/shade button handlers only if buttons are children of title bar. Buttons ARE children of `#title-bar`. A capturing listener at the title-bar level would swallow mousedown on the Close/Shade buttons too, breaking them. Rejected: breaks child controls.
- **Option 5 — `node.replaceWith(node.cloneNode(true))`** — strips all listeners from the node (good) but also strips listeners from child buttons, breaking Close/Shade. Rejected: breaks child controls.
- **Option 6 — MutationObserver pinning style.left/style.top to initial position** — causes flicker (every mousemove mutation triggers a style write and repaint); user would see the window snap back every frame during a drag attempt. Also project-os-4umw proved MutationObserver on Webamp DOM mutations doesn't prevent the viewport expansion. Rejected: flickery UX, doesn't fully prevent expansion.
- **Option 7 — CSS `touch-action: manipulation` or `user-select: none`** — does not disable mouse drag, only touch gestures. Rejected: insufficient.

## Scope

All three Webamp windows must be immobilized:
- `#title-bar` — main window drag handle
- `.playlist-top` (contains `.playlist-top draggable`) — playlist drag handle
- `.equalizer-top` — equalizer drag handle (currently `closed: true` in initial layout, but user can open it)

The `#webamp-mount` container and descendant `.window` elements also carry `"draggable"` in some contexts — spot-check with DevTools to confirm only the three title-bar elements need treatment. The outer window div's `"draggable"` class (equalizer) may also need removal.

## Files

- `src/components/webamp.rs` — only file to change
  - Inside the `init` closure, after `wa.render_when_ready(&target)` (line ~184) and after the non-passive `touchmove` listener (around line 208)
  - Add a `Timeout::new(250, …)` that queries the three selectors and removes `"draggable"` from each found element
  - No new `Rc<RefCell<Option<_>>>` handles needed — the timeout fires once and drops itself

## Must preserve

- [ ] Close button (`#main-window #close`) — closes/hides Webamp
- [ ] Shade/minimize button (`#main-window #shade`) — toggles compact shade mode
- [ ] Minimize button (`#main-window #minimize`) — minimizes to taskbar
- [ ] Double-click on title bar → toggles shade mode (`onDoubleClick={toggleMainWindowShadeMode}`)
- [ ] Main controls: play, pause, stop, prev, next, eject
- [ ] Seek bar (position scrubber)
- [ ] Volume slider
- [ ] Balance slider
- [ ] EQ toggle button
- [ ] Playlist toggle button
- [ ] Shuffle toggle
- [ ] Repeat toggle
- [ ] Playlist window: shade button, close button, track list scroll, all menu items (Add/Remove/Select/Sort/List/Misc)
- [ ] Equalizer window: EQ on/off, auto EQ, preset menu, all band sliders, close/shade
- [ ] Tray icon still toggles Webamp open/closed

## Risks

1. **Timing**: `Timeout::new(250, …)` fires before Webamp finishes injecting its DOM. If `#title-bar` isn't in the DOM yet, `query_selector` returns `None` silently — dragging remains enabled. Mitigation: increase delay or retry with a MutationObserver on `<body>` watching for `#title-bar`'s arrival (one-shot disconnect once found).
2. **Shade mode reshuffle**: When the user toggles shade mode, Webamp may re-render the title bar (React reconciliation). If it replaces the node, the `"draggable"` class removal is lost on the new node. Mitigation: subscribe to the `onWillClose` / or attach a lightweight MutationObserver on `#title-bar`'s parent that re-removes `"draggable"` on child replacement (subtree: true, childList: true). Keep it as a stretch goal — verify first whether shade re-renders destroy the class removal.
3. **Webamp version update**: CDN loads `webamp@^2`; a minor bump could rename classes or change drag logic. This workaround is tied to the class name `"draggable"`. Document the assumption.

## Todo

- [ ] In `src/components/webamp.rs`, after `wa.render_when_ready(&target)`, add a `Timeout::new(250, …)` that calls `document.query_selector` for `"#title-bar"`, `".playlist-top"`, and `".equalizer-top"` and calls `.class_list().remove1("draggable")` on each found element
- [ ] Verify timing: open Webamp, wait, check DevTools — confirm `"draggable"` is gone from all three title bars
- [ ] Verify shade still works: double-click title bar → shade toggles → after shade toggle, confirm `"draggable"` is still absent (i.e. React didn't re-inject it)
- [ ] If shade re-injects `"draggable"`, add a one-shot `MutationObserver` on `document.body` (childList + subtree) that re-removes `"draggable"` whenever `#title-bar` is re-added; disconnect after first match or on component unmount
- [ ] `cargo check --target wasm32-unknown-unknown` passes
- [ ] `cargo clippy --target wasm32-unknown-unknown -- -D warnings` passes (no new warnings)
- [ ] Manual verification on Firefox RDM (Pixel 7 or iPhone 13): taskbar and Clippy no longer drift during/after Webamp interaction; all controls listed in "Must preserve" still work
- [ ] Manual verification on desktop Chrome/Firefox: all listed controls still work; user cannot drag any Webamp window


## Implementation 2026-04-20

Implemented on worktree branch `worktree-agent-ae322004` as commit `ce1c49c` (NOTE: commit subject incorrectly references `project-os-4c2s` — a duplicate ticket that the agent filed by mistake; this ticket is the canonical one).

**Approach chosen:** MutationObserver on `document.body` (childList + subtree). Chose this over the `Timeout` approach because it handles both the async-render timing AND the shade-toggle rebuild case in one mechanism.

**Files changed:**
- `portfolios/src/components/webamp.rs` — new observer attached in the `init` closure, strips `draggable` class from `#main-window #title-bar`, `#playlist-window .playlist-top`, `#equalizer-window .equalizer-top` on every mutation. Observer + Closure stored in `Rc<RefCell<Option<_>>>` handles and disconnected on effect cleanup.
- `portfolios/Cargo.toml` — added `MutationObserver`, `MutationObserverInit`, `MutationRecord`, `Node`, `NodeList`, `DomTokenList` to `web-sys` features.

**Cleanup needed before merge:** drop the stray duplicate ticket file `.beans/project-os-4c2s--disable-webamp-window-drag-by-stripping-draggable.md` that is part of commit `ce1c49c`.

`cargo check --target wasm32-unknown-unknown` and `cargo clippy --target wasm32-unknown-unknown -- -D warnings` both pass (per agent report). Manual Firefox RDM + desktop verification pending.


## Reasons for Scrapping

Abandoned on 2026-04-23. Implementation (branch `worktree-agent-ae322004`) successfully disabled drag via the class-stripping approach, which the user confirmed works ("almost perfect"). However, the player's initial position remained in the bottom-right of mobile viewports regardless of `windowLayout` / inline-style / MutationObserver force-writes. Multiple iterations (position tweak, idempotent observer write) did not move the window, suggesting Webamp's render path is restoring position from a source we did not identify (likely localStorage or an internal default path that bypasses the constructor option).

The drag-disable approach on its own is insufficient without a working position fix. Revisit only with a fresh investigation — start by instrumenting what actually writes `#main-window` inline styles in the browser.

Worktree `worktree-agent-ae322004` and its three commits (`7c152d1`, `f27c9a1`, `d85754f`) were discarded.
