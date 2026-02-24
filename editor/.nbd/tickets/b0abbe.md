+++
title = "Phase 11d: CI integration for test suite"
priority = 5
status = "done"
ticket_type = "task"
dependencies = ["4c2a6e", "cc5ba0"]
+++
Add automated test execution to CI for editor source code:
- Create a new editor-tests.yml workflow
- Run cargo test (native unit tests — Phase 11a)
- Run wasm-pack test --headless --chrome (WASM integration tests — Phase 11b)
- Trigger on push to any branch when files in `editor` directory change
