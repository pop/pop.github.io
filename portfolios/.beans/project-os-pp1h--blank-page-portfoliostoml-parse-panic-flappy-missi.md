---
# project-os-pp1h
title: 'Blank page: portfolios.toml parse panic (flappy missing tech)'
status: completed
type: bug
priority: critical
created_at: 2026-07-27T18:19:01Z
updated_at: 2026-07-27T22:34:55Z
---

Live games.elijah.run and pages.elijah.run/portfolios/ render blank. Root cause: WASM panic at config.rs:94 'failed to parse portfolios.toml: missing field tech'. Game.tech is a required Vec<Tech> but the 'flappy' project has no [[projects.tech]] block. Fix: #[serde(default)] on tech so projects can omit it. Then rebuild + redeploy.

## Summary of Changes
Root cause: `Game.tech` was a required `Vec<Tech>`, but the `flappy` project in portfolios.toml has no `[[projects.tech]]` block. `toml::from_str` panicked at load (config.rs:94, 'missing field tech'), so the Yew app never mounted — blank white page on both games.elijah.run and pages.elijah.run/portfolios/ (identical failure, confirming hosting/Worker were fine).

Fix: added `#[serde(default)]` to `Game.tech` so a project may omit tech (empty vec). Rebuilt bundle, uploaded to pages.elijah.run. Verified live with Firefox: both URLs render the full desktop, no console/page errors.
