---
# project-os-p7i4
title: 'game icon component: clickable desktop icon (image + label)'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Implement the GameIcon component.
- src/components/game_icon.rs: renders game.icon image + game.title label, Win95 icon styling
- Double-click or single-click opens the game window (calls open callback)
- Icons are laid out in a fixed grid; icons cannot be dragged/moved
- Placeholder asset: gray square or generic icon image
