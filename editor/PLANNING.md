# Blog Editor - Planning Document

## Overview

A client-side web application for writing and publishing posts to the
elijah.run blog. Built in Rust with the Yew framework, compiled to WASM,
and running entirely in the browser. A Cloudflare Worker handles the
GitHub OAuth token exchange (the only server-side component).

## Key Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Framework | Yew (Rust/WASM) | User requirement |
| Markdown rendering | markdown-rs | User requirement |
| Auth | GitHub OAuth | Only production auth method |
| Serverless | Cloudflare Workers | OAuth token exchange; Rust/WASM support |
| Content scope | All of `content/` | Blog, fiction, whats-good, backlog, games, root pages |
| Image support | Yes | Upload images via GitHub API |
| Publish flow | Direct merge | Merge editor branch to default branch, no PR |
| Repo location | Same repo | Editor lives in `editor/` subdirectory |
| Hosting | TBD | Decide after app is functional |
| Build tool | Trunk | Standard Yew/WASM bundler |

## Blog Structure Reference

The blog is a Zola static site. Content uses TOML frontmatter (`+++`
delimiters). Posts are either standalone `.md` files or `index.md` inside
a named directory (when images are co-located). The template for new
posts lives at `content/template`.

Example frontmatter:
```toml
+++
title = "Post Title"
date = "2024-12-31"
description = "A short description"
taxonomies.tags = ["tag1", "tag2"]
draft = true
aliases = ["/post-slug"]
+++
```

The GitHub repo is `pop/pop.github.io`. Default branch is `source`.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   Browser                        │
│                                                  │
│  ┌─────────────────────────────────────────────┐ │
│  │           Yew App (WASM)                    │ │
│  │                                             │ │
│  │  ┌─────────┐ ┌────────┐ ┌───────────────┐  │ │
│  │  │ Router  │ │ Auth   │ │ GitHub Client │  │ │
│  │  │         │ │ State  │ │ (REST API)    │  │ │
│  │  └────┬────┘ └───┬────┘ └──────┬────────┘  │ │
│  │       │          │              │            │ │
│  │  ┌────▼──────────▼──────────────▼─────────┐ │ │
│  │  │              Pages                      │ │ │
│  │  │  Login | Dashboard | Editor | Preview   │ │ │
│  │  └────────────────────────────────────────┘ │ │
│  └─────────────────────────────────────────────┘ │
│          │                        │               │
└──────────┼────────────────────────┼───────────────┘
           │                        │
           ▼                        ▼
┌──────────────────┐    ┌───────────────────────┐
│ Cloudflare Worker │    │    GitHub REST API    │
│ (OAuth exchange)  │    │  (repos, contents,    │
│                   │    │   git refs, merges)   │
└──────────────────┘    └───────────────────────┘
```

### Auth Flow

1. User clicks "Login with GitHub"
2. Redirect to `https://github.com/login/oauth/authorize` with
   `client_id`, `redirect_uri`, `scope=repo`
3. GitHub redirects back with `?code=...`
4. Frontend POSTs `code` to Cloudflare Worker
5. Worker exchanges code for access token using client secret
6. Worker returns token to frontend
7. Frontend stores token in `sessionStorage`
8. All subsequent GitHub API calls use this token

For local development: skip OAuth, use a personal access token stored
in an env var or entered manually.

### Editing Workflow

1. User browses content directory via GitHub Contents API
2. User selects a file to edit (or creates new from template)
3. App creates a branch: `editor/{date}-{slug}`
4. Edits are committed to the editor branch on each save
5. User can preview rendered markdown at any time
6. When the user publishes:
   - Merge the editor branch into `source` (default branch)
   - Delete the editor branch
7. When the user discards:
   - Delete the editor branch

### GitHub API Endpoints Used

| Operation | Method | Endpoint |
|---|---|---|
| List directory | GET | `/repos/{owner}/{repo}/contents/{path}` |
| Read file | GET | `/repos/{owner}/{repo}/contents/{path}` |
| Create/update file | PUT | `/repos/{owner}/{repo}/contents/{path}` |
| Delete file | DELETE | `/repos/{owner}/{repo}/contents/{path}` |
| Get branch ref | GET | `/repos/{owner}/{repo}/git/ref/heads/{branch}` |
| Create branch | POST | `/repos/{owner}/{repo}/git/refs` |
| Delete branch | DELETE | `/repos/{owner}/{repo}/git/refs/heads/{branch}` |
| Merge branch | POST | `/repos/{owner}/{repo}/merges` |
| Upload image | PUT | `/repos/{owner}/{repo}/contents/{path}` (base64) |

---

## Project Structure

```
editor/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── src/
│   ├── main.rs
│   ├── app.rs              # Root component, router setup
│   ├── routes.rs           # Route enum definitions
│   ├── components/
│   │   ├── mod.rs
│   │   ├── nav.rs          # Navigation bar
│   │   ├── login.rs        # Login page
│   │   ├── dashboard.rs    # File browser / content listing
│   │   ├── editor.rs       # Text editor + save/publish
│   │   ├── preview.rs      # Markdown preview pane
│   │   └── file_tree.rs    # Directory tree component
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth.rs         # OAuth flow, token management
│   │   └── github.rs       # GitHub API client
│   └── models/
│       ├── mod.rs
│       ├── post.rs         # Post: frontmatter + body
│       └── github.rs       # GitHub API response types
├── worker/
│   ├── Cargo.toml          # or package.json if using JS
│   ├── wrangler.toml
│   └── src/
│       └── lib.rs          # OAuth token exchange endpoint
└── styles/
    └── main.css
```

### Dependencies

**Frontend (Yew app):**

| Crate | Purpose |
|---|---|
| `yew` | Component framework |
| `yew-router` | Client-side routing |
| `markdown` | Markdown-to-HTML (markdown-rs) |
| `gloo-net` | HTTP requests (fetch API) |
| `gloo-storage` | sessionStorage access |
| `gloo-utils` | Browser utilities |
| `serde` + `serde_json` | Serialization |
| `wasm-bindgen` | WASM/JS interop |
| `web-sys` | Web API bindings |
| `base64` | Encode/decode file contents for GitHub API |
| `toml` | Parse/serialize TOML frontmatter |

**Cloudflare Worker:**

| Crate | Purpose |
|---|---|
| `worker` | Cloudflare Workers Rust SDK |
| `serde` + `serde_json` | Serialization |

---

## Implementation Phases

### Phase 1: Project Scaffold & Auth

**Goal:** Yew app boots, user can log in with GitHub.

- [ ] Initialize Cargo workspace in `editor/`
- [ ] Set up Trunk build config (`Trunk.toml`, `index.html`)
- [ ] Create Yew app shell with router (login, dashboard, editor, preview routes)
- [ ] Implement GitHub OAuth redirect (frontend side)
- [ ] Create Cloudflare Worker for token exchange
- [ ] Implement token storage and auth state management
- [ ] Add dev-mode bypass (manual token entry)
- [ ] Verify: user can log in and token is stored

### Phase 2: Content Browsing

**Goal:** Authenticated user can browse the `content/` directory.

- [ ] Implement GitHub API client (list contents, read file)
- [ ] Build dashboard page: directory listing of `content/`
- [ ] Support navigating into subdirectories
- [ ] Display file metadata (name, type, path)
- [ ] Handle GitHub API pagination
- [ ] Verify: user can browse all content directories

### Phase 3: Editing & Branching

**Goal:** User can edit files on an auto-created branch.

- [ ] Implement branch creation via GitHub API
- [ ] On "edit" click: create branch, load file content into textarea
- [ ] Implement save: commit file to editor branch
- [ ] Implement "create new post" from template
  - User picks a section and slug
  - App creates directory + `index.md` from template
- [ ] Implement file deletion
- [ ] Track active editor branch in app state
- [ ] Verify: edits appear as commits on the editor branch

### Phase 4: Markdown Preview

**Goal:** User can preview posts as rendered markdown.

- [ ] Integrate markdown-rs for rendering
- [ ] Build preview component: rendered HTML output
- [ ] Strip TOML frontmatter before rendering
- [ ] Side-by-side or toggle layout (editor | preview)
- [ ] Verify: markdown renders correctly for existing posts

### Phase 5: Image Upload

**Goal:** User can upload images alongside posts.

- [ ] Add file input for image selection
- [ ] Read image as base64 in browser
- [ ] Upload via GitHub Contents API (PUT with base64 content)
- [ ] Insert markdown image reference into editor
- [ ] Show uploaded images in preview
- [ ] Verify: images upload and display correctly

### Phase 6: Publish & Discard

**Goal:** User can merge their branch or discard changes.

- [ ] Implement "Publish": merge editor branch into `source`
- [ ] Delete editor branch after successful merge
- [ ] Implement "Discard": delete editor branch without merging
- [ ] Handle merge conflicts (show error, suggest manual resolution)
- [ ] Verify: published posts appear on default branch

### Phase 7: Polish

**Goal:** Usable, reasonably styled application.

- [ ] Basic CSS styling (readable, functional layout)
- [ ] Error handling and user-facing error messages
- [ ] Loading states for API calls
- [ ] Navigation breadcrumbs
- [ ] Confirm dialogs for destructive actions (delete, discard)
- [ ] Session expiry handling (re-auth flow)

---

## Open Questions

- **Hosting:** Where to deploy the built WASM app. Options include GitHub
  Pages, Cloudflare Pages, or serving from the same repo's static output.
- **Concurrent editing:** What happens if the user has multiple editor
  branches? For now, assume one active branch at a time.
- **Conflict resolution:** If `source` changes while editing, a merge may
  conflict. Initial approach: show an error and suggest the user resolve
  manually on GitHub.
- **Worker deployment:** Need to set up wrangler config and GitHub OAuth
  App credentials (client ID + secret as Worker secrets).

---

## Session Log

### Session 1 (2026-02-12)
- Explored blog repository structure
- Defined requirements and architecture
- Created this planning document
