---
# project-os-yc69
title: Remove Shutdown from start menu
status: todo
type: task
created_at: 2026-04-09T04:37:00Z
updated_at: 2026-04-09T04:37:00Z
---

Remove the 'Shut Down...' button and its preceding separator from the start menu. This involves three areas of change: (1) In games.toml, remove the shutdown_label and shutdown_message fields from the [start_menu] section. (2) In src/config.rs, remove the shutdown_label: String and shutdown_message: String fields from the StartMenu struct. (3) In src/components/start_menu.rs, remove the shutdown_message/on_shutdown callback setup, and remove the <li><hr /></li> separator and the shutdown button <li> from the HTML template.
