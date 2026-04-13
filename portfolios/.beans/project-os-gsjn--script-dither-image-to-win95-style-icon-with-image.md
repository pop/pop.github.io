---
# project-os-gsjn
title: 'Script: dither image to Win95-style icon with ImageMagick'
status: completed
type: task
priority: normal
created_at: 2026-04-09T18:02:03Z
updated_at: 2026-04-09T18:09:09Z
---

Write a bash script (scripts/make-icon.sh) that uses ImageMagick to dither an input image in the style of a 90s Windows 95 icon. Should: reduce to 16 or 256 colors, apply ordered/Floyd-Steinberg dithering, resize to 48x48 or 64x64, output PNG. Usage: ./scripts/make-icon.sh input.png output.png

## Summary of Changes

Created scripts/make-icon.sh using ImageMagick: resize to 48x48, Floyd-Steinberg dithering remapped to the wizard (256-color) palette, output as indexed PNG.
