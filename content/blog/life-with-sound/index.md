+++
title = "Conway's Game of Life with sound"
date = "2026-07-27"
description = "The classic zero-player game, now with color and sound!"
taxonomies.tags = ["gamedev", "prototype", "bevy", "rust", "game-of-life"]
+++

> Play it on itch.io right now! [popgame.itch.io/life-with-sound](https://popgame.itch.io/life-with-sound)

This was a side-quest

wanted to add some sort of orchestral accompaniment, but i don't know much about music!
asked my musicly minded friend Sam and we brainstormed some ideas during our weekly gaming session.

originally thought we could map each cell to a pitch, but it's an infinite grid so... that wouldn't work...

a tiling grid would work though!

added colors to visualize the tones, then customizing the layout, which tones and octaves, speed, added a camera follow feature, all good stuff.

first game written in bevy 0.19 and I really like the new
`bsn! { ... }` syntax is nice and concise. implicit `..default()` is a really nice quality of life improvement, but macro-heavy workflows tend to cause headaches because of magic