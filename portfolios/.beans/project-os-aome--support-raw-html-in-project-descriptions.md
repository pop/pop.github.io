---
# project-os-aome
title: Support raw HTML in project descriptions
status: completed
type: feature
priority: normal
created_at: 2026-04-11T02:27:34Z
updated_at: 2026-04-11T02:28:35Z
---

Project descriptions in portfolios.toml are currently plain strings rendered as escaped text. They should support raw HTML so authors can include links, bold text, etc. Implementation: use Html::from_html_unchecked() in game_window.rs (same pattern as clippy.rs:199), no struct changes needed since description is already a String.

## Summary of Changes

Changed description rendering in game_window.rs from plain text interpolation to Html::from_html_unchecked(), matching the pattern used in clippy.rs. No struct changes needed — description was already a String. portfolios.toml descriptions now use HTML (e.g. Martian Chess links to the Wikipedia article).
