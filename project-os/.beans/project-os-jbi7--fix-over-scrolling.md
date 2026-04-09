---
# project-os-jbi7
title: Fix over-scrolling
status: todo
type: bug
created_at: 2026-04-09T04:37:18Z
updated_at: 2026-04-09T04:37:18Z
---

The page can scroll beyond the viewport, breaking the Win95 desktop illusion. Root causes: (1) index.html has no CSS reset on html/body — neither element has margin:0, padding:0, overflow:hidden, or height:100% set, so the browser default allows scrollbars and overflow. The Yew-rendered root element (likely a <div> wrapping #desktop) also has no explicit sizing constraints. (2) #desktop uses height:100vh and overflow:hidden correctly, but if html/body are taller than the viewport (e.g. due to default margin/padding or the body growing to contain fixed-position children), the 100vh desktop div can still sit inside a scrollable body. (3) The taskbar is position:fixed at bottom:0, which is correct, but windows rendered inside #desktop that are dragged near or beyond the bottom edge can expand the document scroll height if body overflow is not clamped. Fix: add a CSS rule targeting html and body with margin:0; padding:0; overflow:hidden; height:100%; (or height:100vh) so the entire document is locked to the viewport. This should be added either in styles/main.css or as a <style> block in index.html.
