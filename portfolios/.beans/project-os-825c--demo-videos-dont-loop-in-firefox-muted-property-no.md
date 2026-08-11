---
# project-os-825c
title: Demo videos don't loop in Firefox (muted property not set)
status: completed
type: bug
priority: normal
created_at: 2026-07-28T00:09:03Z
updated_at: 2026-07-28T00:14:00Z
---

Video demos use markup attributes autoplay/loop/muted, but Yew sets these via setAttribute after createElement. The muted IDL property does not reflect from the content attribute, so mutedProp stays false (verified via headless check). Firefox blocks/does-not-restart autoplay of an unmuted video, so the demo plays once but does not loop. Fix: set muted + loop as real DOM properties via a NodeRef effect.

## Summary of Changes
Set muted, loop as real DOM properties (and call play()) via a NodeRef use_effect, since Yew's setAttribute path never sets the muted IDL property. Added HtmlMediaElement web-sys feature. Verified headlessly: mutedProp now true (was false), video plays and loops (wraps at end). Built --release, packaged, uploaded to pages.elijah.run/portfolios/.
