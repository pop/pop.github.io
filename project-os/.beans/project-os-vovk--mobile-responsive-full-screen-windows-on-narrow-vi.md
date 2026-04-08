---
# project-os-vovk
title: 'mobile responsive: full-screen windows on narrow viewports'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Make the portfolio work well on mobile screens.
- CSS @media (max-width: 768px): windows use position:fixed; inset:0; width:100%; height:100%; instead of absolute positioned draggable
- On mobile, touch-drag on title bar does nothing (window already full-screen)
- Icon grid reflows to smaller icons on mobile
- Taskbar remains visible and functional on mobile
- Test: open on phone-sized viewport, click icon, window fills screen, X closes it
