---
# editor-vw7o
title: Delete .beads/ directory
status: completed
type: task
priority: low
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Check if .beads/ is tracked in git ('git ls-files .beads/'). If tracked, use 'git rm -r .beads/'. If gitignored, use 'rm -rf .beads/'. This is the final cleanup step after all tickets are migrated.
