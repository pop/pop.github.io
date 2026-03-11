---
# editor-pyud
title: 'Phase 11b: API client tests using wasm-bindgen-test'
status: completed
type: task
priority: normal
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Run integration tests in a headless browser via wasm-pack test --headless --chrome.
- Add wasm-bindgen-test as a dev-dependency
- Create tests/ directory with WASM integration tests
- Mock HTTP responses to test GitHubClient methods:
  - Response parsing (200 with valid JSON, error status codes, malformed JSON)
  - 401 handling (verify 'Unauthorized' error string for auth-expiry detection)
  - list_contents fallback (verify Trees API called when Contents API returns 1000 entries)
  - create_or_update_file request body construction (base64 encoding, SHA inclusion)
- Evaluate gloo-net mocking options — may need a trait-based HTTP abstraction or mockito-style approach for WASM
