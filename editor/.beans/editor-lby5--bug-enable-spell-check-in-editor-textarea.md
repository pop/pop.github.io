---
# editor-lby5
title: 'Bug: enable spell-check in editor textarea'
status: completed
type: bug
priority: normal
created_at: 2026-03-11T17:36:29Z
updated_at: 2026-03-11T17:36:29Z
---

The editor textarea has spellcheck="false" explicitly set (editor.rs ~line 754). Change it to spellcheck="true" so the browser underlines misspelled words. One-line fix.
