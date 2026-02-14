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

### Phase 1: Project Scaffold & Auth ✓

**Goal:** Yew app boots, user can log in with GitHub.

- [x] Initialize Cargo project in `editor/` with Yew 0.21, yew-router, gloo-*, serde, web-sys
- [x] Set up Trunk build config (`Trunk.toml`, `index.html`)
- [x] Create Yew app shell with router (Login, Dashboard, Editor, Preview, NotFound routes)
- [x] Implement GitHub OAuth redirect (frontend side) — login.rs redirects to GitHub, handles `?code=` callback
- [x] Create Cloudflare Worker for token exchange — `worker/src/lib.rs` with CORS, POST `/exchange`
- [x] Implement token storage and auth state management — `AuthContext` via `ContextProvider`, sessionStorage
- [x] Add dev-mode bypass (manual token entry) — collapsible PAT input on login page
- [ ] **TODO:** Set `GITHUB_CLIENT_ID` in `src/components/login.rs`
- [ ] **TODO:** Set `WORKER_URL` in `src/services/auth.rs` after deploying the worker
- [ ] **TODO:** Deploy Cloudflare Worker and configure secrets (`wrangler secret put GITHUB_CLIENT_ID` / `GITHUB_CLIENT_SECRET`)
- [ ] Verify: user can log in and token is stored (works now via dev-mode PAT entry; OAuth pending worker deploy)

### Phase 2: Content Browsing ✓

**Goal:** Authenticated user can browse the `content/` directory.

- [x] Implement GitHub API client (list contents, read file) — `GitHubClient` with `list_contents` and `get_file` methods, reads from `source` branch
- [x] Build dashboard page: directory listing of `content/` — fetches and displays entries on load
- [x] Support navigating into subdirectories — clicking a directory updates state and re-fetches
- [x] Display file metadata (name, type, path) — shows name, dir/file indicator, file size; breadcrumb path navigation
- [ ] **TODO:** Handle GitHub API pagination — Contents API returns max 1000 entries per directory; unlikely to hit for this blog but not handled yet (would need Trees API fallback)
- [ ] Verify: user can browse all content directories (requires valid GitHub token via dev-mode PAT entry)

### Phase 3: Editing & Branching ✓

**Goal:** User can edit files on an auto-created branch.

- [x] Implement branch creation via GitHub API — `get_branch_sha`, `create_branch`, `delete_branch` in `GitHubClient`; `GitRef`/`GitObject` models
- [x] On "edit" click: create branch, load file content into textarea — editor fetches file, decodes base64, populates textarea; branch created on first save
- [x] Implement save: commit file to editor branch — `create_or_update_file` with base64 encoding; creates branch from `source` HEAD on first save
- [x] Implement "create new post" from template — "+ New Post" button on dashboard with section + slug inputs; navigates to editor which generates TOML frontmatter template for 404 paths
- [x] Implement file deletion — Delete button commits file removal to editor branch via `delete_file`
- [x] Track active editor branch in app state — branch name persisted in `sessionStorage` (`editor_branch` key); survives navigation between pages
- [x] Add confirmation dialog before delete — native browser confirm dialog added in Phase 7
- [ ] **TODO:** Handle case where stored branch was deleted externally (e.g. detect 404 on branch and clear sessionStorage)
- [ ] Verify: edits appear as commits on the editor branch (requires valid GitHub token via dev-mode PAT entry)

### Phase 4: Markdown Preview ✓

**Goal:** User can preview posts as rendered markdown.

- [x] Integrate markdown-rs for rendering — `markdown` crate v1.0 with GFM options (tables, strikethrough, task lists, autolinks)
- [x] Build preview component: rendered HTML output — standalone Preview page at `/preview/*path` loads file from `source` branch, renders markdown with `Html::from_html_unchecked`; also integrated into editor
- [x] Strip TOML frontmatter before rendering — `strip_frontmatter` in `models/post.rs` finds `+++` delimiters and returns body only
- [x] Side-by-side or toggle layout (editor | preview) — three-mode toggle (Edit / Preview / Split) in editor toolbar; Split mode shows textarea and rendered output side-by-side with flexbox; page widens to full width in split mode
- [ ] **TODO:** Debounce markdown rendering in split mode for large documents (currently re-renders on every keystroke; not a problem at current content sizes)
- [ ] **TODO:** Add syntax highlighting for code blocks (would need a JS highlight library or WASM-compatible solution)
- [ ] Verify: markdown renders correctly for existing posts (requires valid GitHub token via dev-mode PAT entry)

### Phase 5: Image Upload ✓

**Goal:** User can upload images alongside posts.

- [x] Add file input for image selection — hidden `<input type="file" accept="image/*">` triggered by "Upload Image" button in editor toolbar
- [x] Read image as base64 in browser — async `read_file_as_bytes` helper using FileReader API wrapped in a JS Promise/JsFuture
- [x] Upload via GitHub Contents API (PUT with base64 content) — `upload_binary_file` method on `GitHubClient` encodes raw bytes as base64
- [x] Insert markdown image reference into editor — inserts `![filename](filename)` at cursor position (via `selectionStart`) or appends at end
- [x] Show uploaded images in preview — images already render in markdown preview via existing `.markdown-body img` CSS
- [ ] **TODO:** Handle uploading images when file already exists at path (currently returns a conflict error; could check and pass SHA to overwrite)
- [ ] **TODO:** Support drag-and-drop image upload into the editor textarea
- [ ] **TODO:** Add image size/type validation before upload
- [ ] Verify: images upload and display correctly (requires valid GitHub token via dev-mode PAT entry)

### Phase 6: Publish & Discard ✓

**Goal:** User can merge their branch or discard changes.

- [x] Implement "Publish": merge editor branch into `source` — `merge_branch` method on `GitHubClient` using GitHub Merges API (`POST /repos/{owner}/{repo}/merges`)
- [x] Delete editor branch after successful merge — automatic cleanup after successful merge
- [x] Implement "Discard": delete editor branch without merging — uses existing `delete_branch`, clears sessionStorage
- [x] Handle merge conflicts (show error, suggest manual resolution) — 409 response returns "Merge conflict — resolve manually on GitHub"
- [x] Add confirmation dialog before discard — native browser confirm dialog added in Phase 7
- [ ] **TODO:** Show diff of changes before publishing (would need GitHub Compare API)
- [ ] Verify: published posts appear on default branch (requires valid GitHub token via dev-mode PAT entry)

### Phase 7: Polish ✓

**Goal:** Usable, reasonably styled application.

- [x] Basic CSS styling (readable, functional layout) — system font stack, max-width containers, consistent spacing; all components styled since Phase 1-6
- [x] Error handling and user-facing error messages — styled error banners with pink background and border; all API errors surfaced to user
- [x] Loading states for API calls — loading indicators in dashboard, editor, and preview components
- [x] Navigation breadcrumbs — clickable breadcrumb path in dashboard; back-to-dashboard links in editor and preview
- [x] Confirm dialogs for destructive actions (delete, discard) — native browser confirm dialogs before file deletion and branch discard
- [ ] **TODO:** Session expiry handling (detect 401 from GitHub API and redirect to re-auth)
- [ ] **TODO:** Add keyboard shortcuts (Ctrl+S to save)
- [ ] **TODO:** Add unsaved changes warning when navigating away (beforeunload)

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

### Session 2 (2026-02-12)
- Built Phase 1: full project scaffold with all source files
- App compiles to WASM (`cargo check --target wasm32-unknown-unknown` passes)
- Auth flow: AuthContext with ContextProvider, sessionStorage persistence, OAuth callback handler, dev-mode PAT bypass
- Router: 5 routes with auth guards on protected pages (redirect to login if no token)
- Cloudflare Worker: Rust worker with CORS, exchanges OAuth code for GitHub access token
- Remaining for Phase 1 completion: configure GitHub OAuth App credentials and deploy worker

### Session 3 (2026-02-13)
- Built Phase 2: content browsing
- `GitHubClient` in `services/github.rs`: `list_contents` and `get_file` methods calling GitHub Contents API with Bearer auth, reads from `source` branch explicitly
- `ContentEntry` and `FileContent` models in `models/github.rs` with serde deserialization
- Dashboard rewrite in `components/dashboard.rs`: fetches `content/` on load, sorts dirs-first then alphabetically, click dirs to navigate deeper, click files to open editor route
- Breadcrumb navigation with clickable path segments, "Back" link for parent directory
- Loading and error states for API calls
- CSS styles for content list (bordered rows, hover state, dir/file distinction, file sizes)
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only expected dead-code warnings for Phase 3+ stubs)

### Session 4 (2026-02-13)
- Built Phase 3: editing and branching
- Expanded `GitHubClient` in `services/github.rs`: `get_branch_sha`, `create_branch`, `delete_branch`, `create_or_update_file`, `delete_file`, `get_file` now takes branch parameter; refactored common headers into private `get` helper; added `decode_github_content` utility for base64 decoding
- Added `GitRef`/`GitObject` models for Git Refs API responses; added `base64` and `js-sys` crates, `HtmlTextAreaElement`/`HtmlSelectElement` web-sys features
- Editor rewrite in `components/editor.rs`: loads file from editor branch (if active) or source, falls back to template for new files; textarea editing; Save creates branch on first save (named `editor/{date}-{slug}`) and commits to it; Delete removes file on branch; branch name persisted in sessionStorage
- New post creation: dashboard now has "+ New Post" button with section/slug form; navigates to editor path which auto-generates TOML frontmatter template when file doesn't exist
- CSS styles for editor (textarea, toolbar, save/delete buttons, branch badge, new-file badge) and new-post form
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only expected dead-code warnings for `Post` struct, `ref_name` field, and `delete_branch` method used in Phase 6)

### Session 5 (2026-02-13)
- Built Phase 4: markdown preview
- Added `markdown` crate v1.0 with GFM support; `strip_frontmatter` and `render_markdown` utilities in `models/post.rs`
- Editor now has three-mode view toggle (Edit / Preview / Split) in the toolbar; Preview renders markdown via `Html::from_html_unchecked`; Split mode uses flexbox with both panes at 50% width and wider page max-width
- Standalone Preview component at `/preview/*path` loads file from `source` branch, renders markdown, links to editor
- CSS: view toggle button group, split layout with `.editor-container` flex, `.preview-pane` styling, `.markdown-body` typography (headings, code blocks, tables, blockquotes, lists, images, links, hr, strikethrough)
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only expected dead-code warnings for `ref_name` field and `delete_branch` method used in Phase 6)

### Session 6 (2026-02-14)
- Built Phase 5: image upload
- Added `upload_binary_file` method to `GitHubClient` in `services/github.rs`: takes raw `&[u8]`, base64-encodes, uploads via PUT to Contents API
- Added web-sys features: `Blob`, `File`, `FileList`, `FileReader`, `HtmlElement` for browser file handling
- Editor image upload flow: hidden `<input type="file" accept="image/*">` triggered by "Upload Image" toolbar button; `read_file_as_bytes` async helper wraps FileReader in a `js_sys::Promise` + `JsFuture`; bytes uploaded to same directory as current file; `![name](name)` markdown reference inserted at cursor position
- Helper functions: `sanitize_filename` (lowercase, no spaces), `parent_dir` (extract directory from path), `char_pos_to_byte_offset` (JS selectionStart to Rust byte offset)
- CSS: `.upload-btn` styling (purple accent), `.hidden-file-input` (display:none)
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only expected dead-code warnings for `ref_name` field and `delete_branch` method used in Phase 6)

### Session 6 continued (2026-02-14)
- Built Phase 6: publish & discard
- Added `merge_branch` method to `GitHubClient` in `services/github.rs`: POSTs to GitHub Merges API to merge head branch into base branch; handles 201/204 (success), 404 (not found), 409 (conflict)
- Editor publish flow: "Publish" button merges editor branch into `source`, deletes editor branch, clears sessionStorage branch key, navigates to dashboard; disabled when unsaved changes exist (with hint text)
- Editor discard flow: "Discard" button deletes editor branch, clears sessionStorage, navigates to dashboard
- Added `clear_active_branch` helper for sessionStorage cleanup
- Publish bar UI: `.publish-bar` with green Publish button, red-outline Discard button, contextual hint when changes are unsaved
- CSS: `.publish-bar`, `.publish-btn`, `.discard-btn`, `.publish-hint` styles
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only `ref_name` dead-code warning remaining)

### Session 6 continued (2026-02-14) — Phase 7
- Built Phase 7: polish
- Added native browser confirm dialogs (`window.confirm`) before file Delete and branch Discard
- CSS improvements: global button transitions, `focus-visible` outline styles for accessibility, toolbar `flex-wrap` for small screens
- Error messages: pink background with border for visibility; success/save messages: green background with border
- Added "Back to Dashboard" links in editor and preview pages for easier navigation
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only `ref_name` dead-code warning remaining)
