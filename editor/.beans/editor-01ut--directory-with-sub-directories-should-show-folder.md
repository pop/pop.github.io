---
# editor-01ut
title: Directory with sub-directories should show folder icon
status: completed
type: bug
priority: normal
created_at: 2026-03-11T17:36:31Z
updated_at: 2026-03-11T17:36:31Z
---

When a directory in the file tree contains sub-directories, it should always display the folder icon (📂) regardless of whether it also contains an .md file.

Currently the icon logic at dashboard.rs render_entry() (~line 1628) shows Draft/Published icons based on folder_md_statuses but only falls back to the folder icon if there is no .md file status — it does not check whether the directory has sub-directories.

Fix: Before checking folder_md_statuses, check if the directory has any sub-directory children. If so, always render the folder icon (📂) regardless of .md file status.
