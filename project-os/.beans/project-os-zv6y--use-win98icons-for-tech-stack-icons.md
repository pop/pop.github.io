---
# project-os-zv6y
title: Use win98icons for tech stack icons
status: todo
type: task
created_at: 2026-04-09T04:37:17Z
updated_at: 2026-04-09T04:37:17Z
---

Tech stack icons in [[games.tech]] entries in games.toml currently use emoji (e.g. Rust=🦀, Bevy=🐦️, WebAssembly=🕸️). The game_window.rs component already supports URL-based icons via is_url() — if the icon field starts with 'http' or contains '/', it renders an <img> tag instead of a <span> with emoji text.

Replace emoji icons with classic Win98-style images from https://win98icons.alexmeub.com/. Icons on that site follow the URL pattern: https://win98icons.alexmeub.com/icons/png/<name>_<size>-<variant>.png (e.g. 32x32). Browse the site to find appropriate icons for each tech (Rust, Bevy, WebAssembly) and update the icon field in each [[games.tech]] block in games.toml with the full URL.

No code changes are needed in game_window.rs or game_icon.rs — the is_url() logic already handles both emoji and URL icons correctly. Only games.toml needs updating: replace the emoji strings in the icon fields of all [[games.tech]] entries with win98icons URLs.
