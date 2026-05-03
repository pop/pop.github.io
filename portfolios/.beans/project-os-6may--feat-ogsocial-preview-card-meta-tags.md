---
# project-os-6may
title: 'feat: OG/social preview card meta tags'
status: completed
type: feature
priority: normal
created_at: 2026-04-13T23:22:19Z
updated_at: 2026-04-13T23:26:01Z
---

Add Open Graph + Twitter Card meta tags so the site generates preview cards on Twitter, iMessage, etc. Config lives in portfolios.toml [preview] section; a Trunk post-build hook injects the tags into dist/index.html (crawlers don't run WASM).

## Summary of Changes

- Added `[preview]` section to `portfolios.toml` with title, description, url, image fields
- Added `PreviewConfig` struct to `src/config.rs`
- Hardcoded OG + Twitter Card meta tags directly in `index.html` (og:image left empty for now)
- Decided against a Trunk post-build hook — metadata changes rarely, hardcoding is simpler
