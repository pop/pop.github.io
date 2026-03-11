---
# editor-6qk7
title: Remove bd git hooks from .git/hooks/
status: completed
type: task
priority: low
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

Delete the bd shim hooks from editor/.git/hooks/: pre-commit, prepare-commit-msg, pre-push, post-merge, post-checkout, pre-commit.backup, post-merge.backup. nbd has no hooks subsystem so no replacements are needed.
