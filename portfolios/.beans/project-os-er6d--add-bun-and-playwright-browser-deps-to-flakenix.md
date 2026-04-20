---
# project-os-er6d
title: Add bun and Playwright browser deps to flake.nix
status: in-progress
type: task
priority: high
created_at: 2026-04-20T16:57:27Z
updated_at: 2026-04-20T17:21:13Z
parent: project-os-q8fy
---

Make `bun` and a Playwright-compatible Chromium available in the dev shell so the e2e harness can run in pure Nix.

## Context
- Current `flake.nix` exposes a `devShell` with rust/trunk/etc. (see lines 18–43).
- Playwright on NixOS can't use its bundled browser downloads (dynamic linker mismatch). Standard fix: install `pkgs.playwright-driver.browsers` and export `PLAYWRIGHT_BROWSERS_PATH` + `PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1` from the shell hook.

## Todo
- [x] Add `bun` to `buildInputs`
- [x] Add `playwright-driver.browsers` (or equivalent) to make Chromium available
- [x] Set `shellHook` exporting `PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}` and `PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1` and `PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true`
- [x] Verify `nix flake check` still passes (or at minimum `nix develop --command bun --version` works)
- [x] Commit referencing this ticket

## Non-goals
- Don't touch `package.json` / Playwright config — those land in the next ticket
- Don't add Firefox/WebKit — Chromium only

## Acceptance
After `direnv reload` (or `nix develop`), `bun --version` and `echo $PLAYWRIGHT_BROWSERS_PATH` both succeed.
