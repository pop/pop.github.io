---
# project-os-gsjn
title: 'Script: dither image to Win95-style icon with ImageMagick'
status: todo
type: task
created_at: 2026-04-09T18:02:03Z
updated_at: 2026-04-09T18:02:03Z
---

Write a bash script (scripts/make-icon.sh) that uses ImageMagick to dither an input image in the style of a 90s Windows 95 icon. Should: reduce to 16 or 256 colors, apply ordered/Floyd-Steinberg dithering, resize to 48x48 or 64x64, output PNG. Usage: ./scripts/make-icon.sh input.png output.png
