---
# project-os-5you
title: Serve portfolios site under /portfolios subpath (fix asset paths)
status: completed
type: task
priority: normal
created_at: 2026-07-24T23:30:18Z
updated_at: 2026-07-25T22:48:39Z
parent: project-os-qfcq
---

The site currently assumes it lives at the domain root, so moving to pages.elijah.run/portfolios breaks all asset loads. Fix the three classes of paths so everything resolves under /portfolios/.

## Tasks
- [ ] Trunk.toml: add public_url = "/portfolios/" (fixes generated css/js/wasm tags; keeps trunk serve consistent)
- [ ] portfolios.toml: prefix the leading-slash asset paths (icon/demo/logo/start_icon/skin_url/opus url) with /portfolios; leave https:// links untouched
- [ ] src/components/taskbar.rs: wm-4.png -> /portfolios/wm-4.png
- [ ] index.html: update og:url from https://games.elijah.run
- [ ] Verify: trunk build --release, confirm dist/index.html + assets all reference /portfolios/...

## Revised approach (relative paths, not /portfolios prefix)

Since the site is served at pages.elijah.run/portfolios/index.html, RELATIVE refs resolve correctly under the subpath and also make the games.elijah.run transparent-serve a clean 1:1 prefix (no double-prefix). Canonical domain stays games.elijah.run.

- [x] Trunk.toml: public_url = "./" (emit relative css/js/wasm links)
- [x] portfolios.toml: strip leading slash from asset paths (now relative: martian-chess.png, qsteamp.wsz, its-the-balatro-music.opus, icons, etc.); https:// links untouched; preview.url kept as https://games.elijah.run
- [x] src/components/taskbar.rs: left as relative src="wm-4.png"
- [x] index.html: og:url kept as https://games.elijah.run
- [x] Verify: trunk build --release emits relative paths; no root-absolute asset refs

## Summary of Changes
Made the site path-agnostic so it works at pages.elijah.run/portfolios/ (and transparently at games.elijah.run/). Trunk.toml public_url="./"; portfolios.toml asset paths made relative (leading slash stripped); taskbar.rs kept relative; og:url/preview.url kept as games.elijah.run.

Verified against a real release build: dist/index.html emits ./main-*.css, ./project-os-*.js, module_or_path './...wasm', and ./ modulepreload/preload. Zero bare /asset refs; only external https://unpkg.com/98.css stays absolute.
