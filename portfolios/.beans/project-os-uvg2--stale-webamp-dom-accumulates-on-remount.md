---
# project-os-uvg2
title: Stale Webamp DOM accumulates on remount
status: scrapped
type: bug
priority: normal
created_at: 2026-04-16T23:22:42Z
updated_at: 2026-04-16T23:30:25Z
---

Two extra empty player chrome panels appear stacked beneath the active player. Webamp's `close()` only dispatches a redux CLOSE action — it does NOT `unmount()` the React root or remove the `<div id=webamp>` it appended to our mount target. So when the Webamp component re-mounts (hot reload, effect re-run, etc.), each new instance creates another `<div id=webamp>` while the old one's DOM lingers, producing duplicate chrome.

Fix in src/components/webamp.rs:
- Before constructing a new Webamp instance, clear all children of the mount target (`set_text_content(None)` or remove children) so any leftover DOM from a previous mount is discarded.
- Make sure the cleanup closure also clears the mount target after calling `webamp.close()`, since `close()` alone leaves the node behind.

## Reasons for Scrapping

The stale-DOM hypothesis was wrong. The mount-target clear (both before instantiation and in the cleanup closure) was added and visually verified by the user — the two empty player chrome panels still appeared in the screenshot taken after the change. So the duplicates are NOT leftover DOM from a previous Webamp instance; they live inside the single live instance, most likely something the qsteamp skin (which ships AVS, GEN, GENEX, VIDEO bitmaps per its README) coaxes Webamp into rendering. User decided this isn't worth chasing further; reverted `src/components/webamp.rs` to commit a4dd0e6.
