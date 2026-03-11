---
# editor-s60e
title: 'Phase 11d: CI integration for test suite'
status: completed
type: task
priority: normal
created_at: 2026-03-11T17:36:32Z
updated_at: 2026-03-11T17:36:32Z
blocked_by:
    - editor-5ihq
    - editor-pyud
---

Add automated test execution to CI for editor source code:
- Create a new editor-tests.yml workflow
- Run cargo test (native unit tests — Phase 11a)
- Run wasm-pack test --headless --chrome (WASM integration tests — Phase 11b)
- Trigger on push to any branch when files in `editor` directory change
