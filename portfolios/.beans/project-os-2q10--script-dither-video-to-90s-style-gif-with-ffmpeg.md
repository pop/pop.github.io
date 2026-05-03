---
# project-os-2q10
title: 'Script: dither video to 90s-style GIF with ffmpeg'
status: completed
type: task
priority: normal
created_at: 2026-04-09T18:02:05Z
updated_at: 2026-04-09T18:07:59Z
---

Write a bash script (scripts/make-gif.sh) that uses ffmpeg to convert a video clip into a looping GIF in the style of a 90s web animation. Should: reduce framerate (e.g. 10-15fps), apply palette dithering, limit to 256 colors, keep reasonable dimensions (e.g. max 480px wide), output looping GIF. Usage: ./scripts/make-gif.sh input.mp4 output.gif

## Summary of Changes

Created scripts/make-gif.sh using ffmpeg two-pass palette approach with bayer dithering at 12fps, max 480px wide, looping output.
