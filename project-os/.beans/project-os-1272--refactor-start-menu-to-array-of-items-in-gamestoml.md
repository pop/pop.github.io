---
# project-os-1272
title: Refactor start menu to array of items in games.toml
status: todo
type: task
created_at: 2026-04-09T04:37:38Z
updated_at: 2026-04-09T04:37:38Z
---

Currently the [start_menu] section in games.toml uses individual named fields (about_url, github_url, itchio_url, about_label, github_label, itchio_label) and the StartMenu struct in src/config.rs mirrors these flat fields. The start_menu.rs component hardcodes three <li> buttons referencing each field by name.

Refactor to use a [[start_menu.items]] array of tables, each with:
- icon: String (emoji or image ref)
- title: String (display label)
- url: String (target URL, opens in _blank)

**TOML change**: Replace the individual *_url and *_label fields under [start_menu] with an array of tables, e.g.:
[[start_menu.items]]
icon = '📄'
title = 'Homepage'
url = 'https://elijah.run/about'

[[start_menu.items]]
icon = '🐙'
title = 'GitHub'
url = 'https://github.com/pop'

[[start_menu.items]]
icon = '🎮'
title = 'itch.io'
url = 'https://pop.itch.io'

**Config changes (src/config.rs)**:
- Add a new StartMenuItem struct: { icon: String, title: String, url: String }
- Change StartMenu.items to Vec<StartMenuItem>
- Remove the six individual *_url/*_label fields from StartMenu
- Keep shutdown_label and shutdown_message on StartMenu for now (pending a separate ticket to remove the Shutdown button entirely)

**Component changes (src/components/start_menu.rs)**:
- Replace the three hardcoded <li> buttons with a .map() over props.config.items
- Each item renders as a <li><button> that calls open(item.url.clone()) and displays item.icon + item.title
- Keep the <hr /> separator and Shutdown button as a special case below the mapped items (or drop it if the remove-shutdown ticket lands first)
