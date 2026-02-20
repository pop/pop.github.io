# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Blog editor for elijah.run — a client-side Yew (Rust/WASM) web app that edits and publishes posts to a Zola blog via the GitHub API. A Cloudflare Worker handles OAuth token exchange as the only server-side component. The editor lives in the `editor/` subdirectory of the `pop/pop.github.io` repo.

**Current status:** Phases 1–13 complete and deployed. Phase 11 (automated testing) is the remaining unimplemented phase. See `PLANNING.md` for the full architecture spec and session log.

## Development Environment

Uses Nix Flakes for reproducible tooling. Enter the dev shell:

```
nix develop
```

The shell provides: stable Rust (with `wasm32-unknown-unknown` target), Trunk, wasm-bindgen-cli, and GNU Make.

## Build & Run Commands

```bash
trunk serve          # Dev server with hot reload
trunk build          # Debug build
trunk build --release  # Production build
```

Requires the Nix dev shell for the correct Rust target and Trunk version.

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
