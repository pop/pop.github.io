---
# project-os-oqig
title: Render raw HTML in modal paragraphs
status: completed
type: feature
priority: normal
created_at: 2026-04-09T21:12:55Z
updated_at: 2026-04-09T21:14:18Z
---

Modal paragraphs currently render HTML tags as literal text. Use Html::from_html_unchecked so that <a href> and other HTML in modal_paragraphs is rendered as actual markup.

## Summary of Changes

Changed clippy.rs line 193 to use Html::from_html_unchecked with AttrValue so that HTML in modal_paragraphs (e.g. anchor tags) renders as real markup rather than escaped text.
