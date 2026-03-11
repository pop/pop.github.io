---
# editor-n98o
title: CI build/publish job should not trigger on editor/ or .nbd/ changes
status: completed
type: bug
priority: high
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

The build and publish CI job (main.yml) currently fires on any push to the source branch, including changes to editor/ or .nbd/ directories. It should only run when content/, templates/, or config.toml changes.

Relevant file: .github/workflows/main.yml (in parent repo root, not editor/)
Fix: Add paths-ignore or paths filter to the workflow's push trigger to skip editor/ and .nbd/ directories.

Example fix:
on:
  push:
    branches: [source]
    paths:
      - 'content/**'
      - 'templates/**'
      - 'config.toml'
