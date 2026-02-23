+++
title = "Phase 11d: CI integration for test suite"
priority = 5
status = "todo"
ticket_type = "task"
dependencies = ["4c2a6e", "cc5ba0"]
+++
Add automated test execution to CI:
- Add a test job to the GitHub Actions workflow (or create a new editor-tests.yml workflow)
- Run cargo test (native unit tests — Phase 11a)
- Run wasm-pack test --headless --chrome (WASM integration tests — Phase 11b)
- Trigger on push to editor/** branches alongside the existing zola build check
- Add a Makefile target: make test that runs both test suites locally