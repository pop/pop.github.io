# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Blog editor for elijah.run — a client-side Yew (Rust/WASM) web app that edits and publishes posts to a Zola blog via the GitHub API. A Cloudflare Worker handles OAuth token exchange as the only server-side component. The editor lives in the `editor/` subdirectory of the `pop/pop.github.io` repo.

### Session Rules

These are hard constraints enforced from recurring friction patterns. Violations will cause build failures, deploy errors, or lost work.

1. **Never wrap commands in `nix develop`** — the dev shell is already active via direnv. Run `cargo`, `trunk`, `tofu`, etc. directly.
2. **Never use `#[allow(dead_code)]`** — remove unused code instead. Suppression hides rot.
3. **Search only within the project root** — never glob or grep globally across the filesystem. Scope to relevant subdirectories (`src/`, `content/`, etc.).
4. **Always deploy to production** — use `wrangler pages deploy --branch=source` for the frontend. Never deploy to preview unless explicitly asked.
5. **`nix develop --command` is for CI only** — do not use it in interactive sessions; the shell is already loaded.
6. **Verify `nbd` is on PATH before use** — run `which nbd` if there is any doubt about the binary being available.

## Task Tracking with nbd

`nbd` is a CLI tool for managing work tickets, designed for agent workflows.

### Initialisation

```sh
nbd init
```

Run once in the project root. Creates `.nbd/tickets/`. Safe to run multiple times.

### Core commands

```sh
# Create a new ticket (use --ftype md for a human-readable body)
nbd create --title "Add OAuth login" --type feature --priority 7 --ftype md

# List all open tickets (sorted by priority)
nbd list

# Read a specific ticket
nbd read <id>

# Update a ticket
nbd update <id> --status in_progress
nbd update <id> --status done
```

### Finding what to work on

```sh
# All tickets that are unblocked and ready to start
nbd ready

# The single highest-priority unblocked ticket
nbd next
```

### Workflow

1. **Before starting** — create a ticket: `nbd create --title "..." --json`
2. **When starting** — mark it in progress: `nbd update <id> --status in_progress`
3. **When done** — mark it complete: `nbd update <id> --status done`

### Guidelines

- **Always pass `--json`** to every command for structured, unambiguous output.
- **Always pass `--ftype md`** when creating tickets — markdown format keeps the body human-readable.
- Use `jq` to parse and transform JSON output when needed.
- Priority scale 0–10: use **7–9** for bugs, **5** for normal tasks, **3** for nice-to-haves.
- `--type` choices: `project`, `feature`, `task`, `bug`.
- Use `--deps id1,id2` to express blockers — tickets that must be done first.
- Create tickets *before* starting non-trivial tasks, not after.

## Development Environment

Uses Nix Flakes for reproducible tooling. The dev shell is activated automatically via `direnv` — all tools (`cargo`, `trunk`, `wasm-bindgen-cli`, `make`) are available directly in the shell without any wrapper.

Do NOT wrap commands in `nix develop` - the dev shell is already active when running commands in this project.

## Validation

Validate changes with `cargo` for valid syntax, formatting, and linting:

```bash
cargo fmt
cargo check
cargo clippy
```

All checks should pass. If they do not, fix any issues reported by these commands.
Dead code should be deleted, not ignored.

## Build & Run Commands

```bash
trunk serve          # Dev server with hot reload
trunk build          # Debug build
trunk build --release  # Production build
cargo check --target wasm32-unknown-unknown  # Type-check
```

## Testing

### WASM tests (browser)

WASM integration tests live in `tests/wasm.rs` and run in a headless Chrome browser via `wasm-pack`.

```bash
wasm-pack test --headless --chrome --chromedriver $(which chromedriver)
```

**Always pass `--chromedriver $(which chromedriver)`** — without it, wasm-pack tries to download a matching driver from Google's CDN, which 404s for Chrome versions above 114. The Nix devShell provides `chromium` and `chromedriver` at matching versions; use them directly.

### Native unit tests

Pure-Rust logic (frontmatter parsing, slug generation, etc.) is tested with standard `#[cfg(test)]` modules and runs natively:

```bash
cargo test
```

Do NOT prefix commands with `nix develop --command ...`. Run them directly.

Prefer to run a single command over chaining commands together with `&&`.
For example:
```
git add file1 file2 file3
git commit -m 'useful commit message goes here'
```
is prefered to `git add file1 file2 fil3 && git commit -m 'useful commit message goes here'`.

## Architecture

- **Frontend:** Yew framework compiled to WASM, bundled with Trunk. Uses yew-router for client-side routing, gloo-net for HTTP, gloo-storage for sessionStorage.
- **Backend:** Single Cloudflare Worker (`worker/` subdirectory) that exchanges OAuth codes for GitHub access tokens. Keeps the client secret server-side.
- **Data layer:** Authenticated reads use the GitHub GraphQL API (`/graphql`); anonymous reads and all writes use the GitHub REST API (Contents API, Git Refs API, Merges API). `compare_branches`, `get_check_runs`, and commit-date fetching stay as REST.

### Key flows

- **Auth:** GitHub OAuth redirect → Cloudflare Worker exchanges code for token → token stored in sessionStorage. Dev builds only: manual personal access token entry (hidden in release via `cfg!(debug_assertions)`).
- **Editing:** Create branch `editor/{date}-{slug}` → commit edits to branch → merge to `source` branch on publish (or delete branch on discard).

### Source layout

```
src/
├── main.rs              # Entry point
├── app.rs               # Root component, router
├── routes.rs            # Route enum definitions
├── components/          # nav.rs, login.rs, dashboard.rs, editor.rs, preview.rs
├── services/            # auth.rs (OAuth), github.rs (API client — REST + GraphQL)
└── models/              # post.rs (frontmatter + body), github.rs (API types)
worker/                  # Cloudflare Worker for OAuth token exchange
```

## Deployment

### Frontend (Cloudflare Pages)

```bash
trunk build --release
# Deploy preview release
wrangler pages deploy
# Deploy production release
wrangler pages deploy --branch=source
```

The Pages project is a Direct Upload project (no git integration). Build locally with Trunk, then deploy the `dist/` directory. Production URL: https://editor.elijah.run

### OAuth Worker (Cloudflare Workers)

```bash
cd worker
wrangler deploy
```

The worker name is `blog-editor-oauth`. Secrets (`GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`) are set via `wrangler secret put`.

### Infrastructure

OpenTofu configs live in `infra/`. The Pages project, custom domain DNS, and worker are all managed there. Run `tofu apply` from `infra/` to update infrastructure.

## Blog Content Conventions

The blog is a Zola static site. Content uses TOML frontmatter with `+++` delimiters. Posts are either standalone `.md` files or `index.md` inside a named directory (for co-located images). The repo's default branch is `source`. New post templates live at `content/template`.

## Key Dependencies

- `yew` / `yew-router` — component framework and routing
- `markdown` (markdown-rs) — markdown rendering
- `gloo-net` / `gloo-storage` / `gloo-utils` — browser API wrappers
- `serde` / `serde_json` / `toml` — serialization
- `base64` — GitHub API file content encoding (REST path; GraphQL returns plain text)
- `futures` — `join_all` for parallel commit-date fetching
- `js-sys` / `wasm-bindgen` / `wasm-bindgen-futures` — JS interop and async
- `log` / `wasm-logger` — logging in WASM
- `web-sys` — browser APIs (FileReader, DragEvent, KeyboardEvent, etc.)
- `worker` — Cloudflare Workers Rust SDK (for the OAuth worker)

## Deployment Notes

For Cloudflare deployments: production deploys go to the production branch/environment.
Do not deploy to preview when production is requested. Use `wrangler pages deploy --branch=source` or the OpenTofu/Terraform config for production. (The production branch is `source`, not `main`.)

## Code Search / File Operations

When searching or modifying content files, scope to the `content/` directory and be branch-aware.
Do not search globally across the entire filesystem.

## Rust Conventions

Prefer removing dead code over suppressing warnings with `#[allow(dead_code)]`.
This project uses Nix with rust-overlay, NOT rustup.
