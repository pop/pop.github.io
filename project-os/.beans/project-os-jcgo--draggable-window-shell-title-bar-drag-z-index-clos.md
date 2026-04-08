---
# project-os-jcgo
title: 'draggable window shell: title bar drag, z-index, close button'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Generic Win95-style window with drag and z-index management.
- src/components/window.rs: takes title, z_index, pos, on_close, on_focus, children props
- Title bar: game title left, X button right (Win95 style from win95.css)
- Dragging: onmousedown on title bar records offset; document-level onmousemove updates pos; onmouseup clears drag. Use gloo-events EventListener on document.
- Touch: ontouchstart/ontouchmove/ontouchend equivalents
- Clicking anywhere on window calls on_focus (increments z_counter, assigns to this window)
- Position via inline style: position:absolute, left, top, z-index
- Desktop: position:relative, overflow:hidden (windows clipped to desktop area)
