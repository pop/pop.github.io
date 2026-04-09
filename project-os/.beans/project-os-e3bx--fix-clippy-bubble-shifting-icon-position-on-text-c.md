---
# project-os-e3bx
title: Fix Clippy bubble shifting icon position on text change
status: in-progress
type: bug
priority: normal
created_at: 2026-04-09T04:37:29Z
updated_at: 2026-04-09T04:56:54Z
---

The .clippy-widget uses display:flex with flex-direction:column and align-items:center. The widget is anchored bottom-right via position:fixed. When the speech bubble content changes to a quote with different text width, the bubble grows/shrinks horizontally, and because align-items:center distributes children relative to the widest element, the overall widget box shifts left/right, dragging the icon with it.\n\nFix: give .clippy-icon position:relative with a fixed or known anchor point, and float the .clippy-bubble above it independently. The cleanest approach is to make .clippy-widget position:fixed (already done) but give .clippy-icon a stable right-anchor, then make .clippy-bubble position:absolute with bottom:100% and right:50% transform:translateX(50%) (or left:50% transform:translateX(-50%)) so it centers above the icon without affecting the icon's layout. This requires setting .clippy-widget to position:relative internally (or using a wrapper), and ensuring the bubble does not participate in normal flow that can shift the icon. The tail pseudo-element (::after) already uses left:50% translateX(-50%) centering so it should remain correct. The icon should be the layout anchor; the bubble should be an absolutely-positioned overlay above it.
