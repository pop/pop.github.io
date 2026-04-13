---
# project-os-yjdc
title: Add hyperlink support to project technologies
status: completed
type: feature
priority: normal
created_at: 2026-04-11T02:27:37Z
updated_at: 2026-04-11T02:28:38Z
---

Tech items in portfolios.toml only have name and icon fields. Add an optional url field to the Tech struct so tech names can link to their homepage. Implementation: add url: Option<String> to Tech struct in config.rs, update game_window.rs to wrap tech name in <a> tag when url is present, add example URLs in portfolios.toml.

## Summary of Changes

Added url: Option<String> to the Tech struct in config.rs. Updated game_window.rs to conditionally wrap the tech name in an <a target="_blank"> tag when a URL is present. Added urls for Rust, Bevy, and WebAssembly in all project entries in portfolios.toml.
