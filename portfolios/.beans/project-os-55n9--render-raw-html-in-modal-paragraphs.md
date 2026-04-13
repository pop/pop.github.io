---
# project-os-55n9
title: Render raw HTML in modal paragraphs
status: in-progress
type: feature
created_at: 2026-04-09T21:12:51Z
updated_at: 2026-04-09T21:12:51Z
---

Modal paragraphs currently render HTML tags as literal text. Use Html::from_html_unchecked so that <a href> and other HTML in modal_paragraphs is rendered as actual markup.
