---
# project-os-d25a
title: 'taskbar: open-window buttons + live clock + Start button'
status: todo
type: task
priority: normal
created_at: 2026-04-08T04:55:31Z
updated_at: 2026-04-08T04:55:31Z
parent: project-os-zsw7
---

Implement the Win95 taskbar pinned to the bottom.
- src/components/taskbar.rs: fixed bottom bar using win95.css panel styles
- Start button on far left (triggers start menu show/hide)
- Center section: one button per open window (shows game title); clicking brings window to front (on_focus callback)
- Clock on far right: formatted HH:MM, updates every 1s via gloo_timers Interval
- Integrate with WindowManager state in App
