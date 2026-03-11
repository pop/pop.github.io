---
# editor-c8dj
title: 'Testing: add test job to GitHub Actions CI'
status: completed
type: task
priority: low
created_at: 2026-03-11T17:36:32Z
updated_at: 2026-03-11T17:36:32Z
blocked_by:
    - editor-usl7
    - editor-s7we
---

Add automated test execution to CI.\n\n- Add test job to GitHub Actions workflow (or create new workflow)\n- Run cargo test (native unit tests) and wasm-pack test --headless --chrome (WASM integration tests)\n- Run on push to editor/** branches alongside existing zola build check\n- Add Makefile target: make test that runs both test suites locally
