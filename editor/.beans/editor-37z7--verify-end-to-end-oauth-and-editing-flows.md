---
# editor-37z7
title: 'Verify: end-to-end OAuth and editing flows'
status: todo
type: task
priority: high
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

Verify all core flows work end-to-end with a real GitHub token.\n\nPhase 1: user can log in and token is stored (OAuth flow functional)\nPhase 2: user can browse all content directories\nPhase 3: edits appear as commits on the editor branch\nPhase 4: markdown renders correctly for existing posts\nPhase 5: images upload and display correctly\nPhase 6: published posts appear on default branch\nPhase 13: unauthenticated browsing, preview, and editor viewing work end-to-end\n\nRequires valid GitHub token via dev-mode PAT entry or full OAuth flow.
