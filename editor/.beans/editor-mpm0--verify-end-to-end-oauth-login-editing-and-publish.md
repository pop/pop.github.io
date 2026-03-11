---
# editor-mpm0
title: 'Verify end-to-end: OAuth login, editing, and publish flows'
status: todo
type: task
priority: high
created_at: 2026-03-11T17:36:30Z
updated_at: 2026-03-11T17:36:30Z
---

End-to-end verification for Phases 1-6. Requires a valid GitHub token (dev-mode PAT entry). Checklist:
- Phase 1: user can log in via OAuth and token is stored in sessionStorage
- Phase 2: user can browse all content directories
- Phase 3: edits appear as commits on the editor branch
- Phase 4: markdown renders correctly for existing posts
- Phase 5: images upload and display correctly in preview
- Phase 6: published posts appear on the default (source) branch after merge
