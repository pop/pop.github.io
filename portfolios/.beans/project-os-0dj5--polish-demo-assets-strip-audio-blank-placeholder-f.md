---
# project-os-0dj5
title: 'Polish demo assets: strip audio, blank placeholder for Set'
status: completed
type: task
created_at: 2026-04-16T23:11:09Z
updated_at: 2026-04-16T23:11:09Z
---

Demo videos auto-play and shouldn't bring sound — strip audio so they're smaller and never wake speakers. Set has no demo video yet, so swap to a blank placeholder image until one is recorded.

## Summary of Changes

- public/flappy.mp4: re-encoded with audio track dropped (266KB → 229KB)
- public/martian-chess.mp4: re-encoded with audio track dropped (~252KB)
- public/blank.png: new 1x1-ish placeholder image
- portfolios.toml: `set.demo` swapped from `/set.mp4` to `/blank.png`
