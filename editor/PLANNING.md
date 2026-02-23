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
| Public viewing | No login required | Browsing and previewing content is unauthenticated; login only needed for editing/publishing |
| Serverless | Cloudflare Workers | OAuth token exchange; Rust/WASM support |
| Content scope | All of `content/` | Blog, fiction, whats-good, backlog, games, root pages |
| Image support | Yes | Upload images via GitHub API |
| Publish flow | Direct merge | Merge editor branch to default branch, no PR |
| Repo location | Same repo | Editor lives in `editor/` subdirectory |
| Hosting | Cloudflare Pages | Custom domain `editor.elijah.run`; infrastructure managed via OpenTofu |
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
in an env var or entered manually (debug builds only).

**Public access:** The `pop/pop.github.io` repo is public, so the GitHub
Contents API works without authentication for read-only operations.
Browsing content and previewing posts should work without login. Each
page should show a "Login with GitHub" button; once authenticated, the
user gains access to editing, publishing, and other write operations.

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
- [x] **TODO:** Set `GITHUB_CLIENT_ID` in `src/components/login.rs` — configured via OpenTofu, value in `terraform.tfvars`
- [x] **TODO:** Set `WORKER_URL` in `src/services/auth.rs` after deploying the worker — set to `https://blog-editor-oauth.elijah.run`
- [x] **TODO:** Deploy Cloudflare Worker and configure secrets — deployed via OpenTofu (`cloudflare_worker` + `cloudflare_worker_version` + `cloudflare_workers_deployment`); secrets set as `secret_text` bindings
- [ ] Verify: user can log in and token is stored (OAuth flow functional; needs end-to-end verification)

### Phase 2: Content Browsing ✓

**Goal:** Authenticated user can browse the `content/` directory.

- [x] Implement GitHub API client (list contents, read file) — `GitHubClient` with `list_contents` and `get_file` methods, reads from `source` branch
- [x] Build dashboard page: directory listing of `content/` — fetches and displays entries on load
- [x] Support navigating into subdirectories — clicking a directory updates state and re-fetches
- [x] Display file metadata (name, type, path) — shows name, dir/file indicator, file size; breadcrumb path navigation
- [x] Handle GitHub API pagination — `list_contents` detects 1000-entry truncation and falls back to recursive Git Trees API; converts TreeEntry to ContentEntry for seamless dashboard display
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
- [x] Handle case where stored branch was deleted externally — load effect verifies branch SHA before loading file; clears sessionStorage on 404
- [ ] Verify: edits appear as commits on the editor branch (requires valid GitHub token via dev-mode PAT entry)

### Phase 4: Markdown Preview ✓

**Goal:** User can preview posts as rendered markdown.

- [x] Integrate markdown-rs for rendering — `markdown` crate v1.0 with GFM options (tables, strikethrough, task lists, autolinks)
- [x] Build preview component: rendered HTML output — standalone Preview page at `/preview/*path` loads file from `source` branch, renders markdown with `Html::from_html_unchecked`; also integrated into editor
- [x] Strip TOML frontmatter before rendering — `strip_frontmatter` in `models/post.rs` finds `+++` delimiters and returns body only
- [x] Side-by-side or toggle layout (editor | preview) — three-mode toggle (Edit / Preview / Split) in editor toolbar; Split mode shows textarea and rendered output side-by-side with flexbox; page widens to full width in split mode
- [x] Debounce markdown rendering in split mode — 200ms debounce via async sleep + generation counter; `rendered_html` state avoids re-rendering on every keystroke
- [x] Add syntax highlighting for code blocks — highlight.js loaded via CDN; `use_effect` calls `hljs.highlightElement` on unhighlighted `<code>` blocks after each preview render
- [ ] Verify: markdown renders correctly for existing posts (requires valid GitHub token via dev-mode PAT entry)

### Phase 5: Image Upload ✓

**Goal:** User can upload images alongside posts.

- [x] Add file input for image selection — hidden `<input type="file" accept="image/*">` triggered by "Upload Image" button in editor toolbar
- [x] Read image as base64 in browser — async `read_file_as_bytes` helper using FileReader API wrapped in a JS Promise/JsFuture
- [x] Upload via GitHub Contents API (PUT with base64 content) — `upload_binary_file` method on `GitHubClient` encodes raw bytes as base64
- [x] Insert markdown image reference into editor — inserts `![filename](filename)` at cursor position (via `selectionStart`) or appends at end
- [x] Show uploaded images in preview — images already render in markdown preview via existing `.markdown-body img` CSS
- [x] Handle uploading images when file already exists at path — checks existing file SHA before upload, passes SHA to overwrite
- [x] Support drag-and-drop image upload into the editor textarea — ondragover/ondragleave/ondrop on editor container with visual indicator
- [x] Add image size/type validation before upload — validates MIME type (png, jpeg, gif, webp, svg+xml) and file size (10 MB max) before reading bytes
- [ ] Verify: images upload and display correctly (requires valid GitHub token via dev-mode PAT entry)

### Phase 6: Publish & Discard ✓

**Goal:** User can merge their branch or discard changes.

- [x] Implement "Publish": merge editor branch into `source` — `merge_branch` method on `GitHubClient` using GitHub Merges API (`POST /repos/{owner}/{repo}/merges`)
- [x] Delete editor branch after successful merge — automatic cleanup after successful merge
- [x] Implement "Discard": delete editor branch without merging — uses existing `delete_branch`, clears sessionStorage
- [x] Handle merge conflicts (show error, suggest manual resolution) — 409 response returns "Merge conflict — resolve manually on GitHub"
- [x] Add confirmation dialog before discard — native browser confirm dialog added in Phase 7
- [x] Show diff of changes before publishing — `compare_branches` method using GitHub Compare API; "Publish" shows diff panel with file patches, additions/deletions stats; "Confirm Publish" to merge
- [ ] Verify: published posts appear on default branch (requires valid GitHub token via dev-mode PAT entry)

### Phase 7: Polish ✓

**Goal:** Usable, reasonably styled application.

- [x] Basic CSS styling (readable, functional layout) — system font stack, max-width containers, consistent spacing; all components styled since Phase 1-6
- [x] Error handling and user-facing error messages — styled error banners with pink background and border; all API errors surfaced to user
- [x] Loading states for API calls — loading indicators in dashboard, editor, and preview components
- [x] Navigation breadcrumbs — clickable breadcrumb path in dashboard; back-to-dashboard links in editor and preview
- [x] Confirm dialogs for destructive actions (delete, discard) — native browser confirm dialogs before file deletion and branch discard
- [x] Session expiry handling — all API methods detect 401, clear auth token, redirect to login
- [x] Add keyboard shortcuts — Ctrl+S / Cmd+S triggers save via document keydown listener
- [x] Add unsaved changes warning — beforeunload event with use_mut_ref tracking dirty state

### Phase 8: Dashboard Enhancements

**Goal:** Richer content browsing with search, sort, caching, and multi-branch support.

#### 8a. Search / filter ✓

- [x] Add a text input above the content list that filters entries by name as the user types — `.filter-input` in `.filter-sort-bar` with `oninput` callback
- [x] Filter should be case-insensitive substring match — `filter.to_lowercase()` compared against `e.name.to_lowercase()`
- [x] Clear button to reset the filter — `.clear-filter-btn` with × character, absolutely positioned inside filter input
- [x] Filter state resets when navigating into a subdirectory — `filter_text.set(String::new())` in `on_navigate`, `on_navigate_up`, and breadcrumb callbacks

#### 8b. Sort options ✓

- [x] Add a sort toggle/dropdown: **Alphabetical** (default) or **Last modified** — `SortMode` enum with `Alphabetical`/`LastModified`; two toggle buttons ("A–Z" / "Recent") in `.sort-bar`
- [x] Alphabetical: current behavior (dirs first, then A-Z) — default sort from API fetch preserved
- [x] Last modified: fetch last commit date per file using the Commits API (`GET /repos/{owner}/{repo}/commits?path={file}&per_page=1`), sort most recent first — `get_commit_dates_bulk` uses `futures::join_all` for parallel fetching; lazy-loaded only when "Recent" clicked
- [x] Cache commit dates alongside file entries so re-sorting doesn't re-fetch — `CachedListing.commit_dates: Option<HashMap<String, f64>>` stored in sessionStorage alongside entries
- [x] Show the last-modified date in the entry row when sorting by time — `.entry-date` span in `render_entry` with `format_date(epoch_ms)` helper
- [x] Dirs-first ordering should still apply within each sort mode — both sort modes use `type_ord` (dir < file) as primary sort key

#### 8c. Cache list results ✓

- [x] Cache directory listings in `sessionStorage` keyed by path, with a TTL (e.g. 5 minutes) — `CachedListing` struct with entries + timestamp, 5-minute TTL checked against `js_sys::Date::now()`
- [x] On navigation, serve from cache if within TTL; otherwise fetch from API — dashboard effect checks `get_cached_listing` before API call
- [x] Auto-invalidate cache for a path after save, publish, delete, or discard touches that directory — editor calls `invalidate_cache(parent_dir)` on save/delete, `invalidate_all_caches` on discard
- [x] Add a visible "Refresh" button in the dashboard header to force re-fetch — refresh button increments `force_refresh` counter which bypasses cache
- [x] Cache should store entries + commit dates together so sorted views are instant on revisit — `CachedListing` includes `commit_dates: Option<HashMap<String, f64>>`; `get_cached_commit_dates`/`set_cached_commit_dates` helpers read/write alongside entries

#### 8d. Branch selector ✓

- [x] Add a branch picker in the dashboard header (dropdown or sidebar panel) — toggle button shows/hides branch list panel
- [x] Fetch all branches matching `editor/*` from the Git Refs API (`GET /repos/{owner}/{repo}/git/matching-refs/heads/editor/`) — `list_editor_branches` method on `GitHubClient`
- [x] Display each branch with its name and date (extracted from branch name convention `editor/{date}-{slug}`) — branch list shows `editor/` prefix stripped names
- [x] Clicking a branch sets it as the active branch in sessionStorage and reloads the dashboard — updates `editor_branch` key, invalidates all caches, increments refresh counter
- [x] Show the currently active branch prominently (highlighted or with a badge) — purple badge in dashboard header with branch name
- [x] Allow "deselecting" a branch to return to browsing the `source` branch directly — "View source" and "x" buttons clear active branch
- [x] `list_contents` now accepts optional branch parameter (was hardcoded to `source`) — dashboard passes active branch to API
- [ ] **TODO:** Consider showing commit count or last activity per branch (extra API calls)

### Phase 9: Preview Enhancements ✓

**Goal:** Display structured frontmatter alongside rendered content.

- [x] Parse TOML frontmatter into key-value pairs (extend `models/post.rs` with a `parse_frontmatter` function using the `toml` crate) — `extract_frontmatter`, `parse_frontmatter`, `flatten_toml`, `format_toml_value` functions added; nested tables flattened with dot notation (e.g. `taxonomies.tags`)
- [x] Display frontmatter as a table above the rendered markdown in both the editor preview pane and the standalone Preview page — `.frontmatter-table` rendered with `<tr><td class="fm-key">` / `<td class="fm-value">` rows
- [x] Table should show all fields: title, date, description, taxonomies, draft status, aliases, etc. — all TOML value types handled (strings, booleans, integers, floats, arrays as comma-separated, datetimes, nested tables)
- [x] Style the frontmatter table distinctly from the markdown body (e.g. muted background, smaller font) — `#f6f8fa` background, `0.85rem` font, bordered, bold keys
- [x] In the editor, the frontmatter table should update live as the user edits the `+++` block — `frontmatter_fields` state updated in same debounced render effect alongside `rendered_html`
- [x] Added `toml = "0.8"` to Cargo.toml dependencies

### Phase 10: CI Integration ✓

**Goal:** Automated zola build checks on editor branches, with publish gating in the UI.

#### 10a. GitHub Actions workflow ✓

- [x] Create `.github/workflows/editor-check.yml` in `pop/pop.github.io` — workflow file at repo root
- [x] Trigger: `push` to branches matching `editor/**`
- [x] Job: checkout repo (with LFS + submodules), install Zola 0.19.2 via `taiki-e/install-action`, run `zola build`
- [x] Keep the workflow minimal (no deploy, just validate the build)

#### 10b. CI status in the editor UI ✓

- [x] Add a `get_check_runs` method to `GitHubClient` using the Checks API (`GET /repos/{owner}/{repo}/commits/{ref}/check-runs`)
- [x] Add `CheckRunsResponse` / `CheckRun` models (id, name, status, conclusion, html_url)
- [x] In the editor publish bar, fetch the CI status for the current branch head — `fetch_ci_status` async helper called on mount
- [x] Display CI state: pending (hourglass, yellow), success (checkmark, green), failure (X, red with link), or no checks yet (hidden)
- [x] **Block the Publish button** when CI is pending or has failed — `ci_blocks_publish` flag added to disabled conditions
- [x] On failure, show a link to the failed CI run (`html_url` from the check run) so the user can inspect the build log
- [x] Poll CI status every 15 seconds via `setInterval`; stops polling when CI reaches terminal state (success/failure) or component unmounts
- [x] When no check runs exist: allow publish (treated as `CiState::None`, does not block)

### Phase 11: Automated Testing

**Goal:** Add unit and integration tests to catch regressions, especially before the GraphQL migration refactor.

#### 11a. Unit tests (pure functions)

These are standard `#[cfg(test)]` modules — no browser or WASM runtime needed.

- [ ] `models/post.rs` — test `strip_frontmatter` (with/without frontmatter, empty input, unclosed delimiters) and `render_markdown` (basic markdown, GFM features, frontmatter stripping)
- [ ] `services/github.rs` — test `decode_github_content` (valid base64, base64 with whitespace/newlines, invalid input, empty string)
- [ ] `components/editor.rs` — test `slug_from_path` (index.md, _index.md, standalone .md, nested paths, edge cases), `title_from_slug` (hyphenated, single word, empty), `sanitize_filename` (spaces, special chars, uppercase, unicode), `char_pos_to_byte_offset` (ASCII, multi-byte UTF-8, out-of-bounds), `parent_dir` (nested path, root-level file, no slash), `generate_template` (verify frontmatter structure)
- [ ] `components/dashboard.rs` — test `format_size` (bytes, KB, MB boundaries)
- [ ] Extract testable pure functions from component files into a shared `utils.rs` module to simplify testing and avoid pulling in Yew dependencies

#### 11b. API client tests (`wasm-bindgen-test`) ✓ (infrastructure + evaluation)

These run in a headless browser via `wasm-pack test --headless --chrome`.

- [x] Add `wasm-bindgen-test = "0.3"` as a dev-dependency
- [x] Create `tests/wasm.rs` with `#![cfg(target_arch = "wasm32")]` gate and `wasm_bindgen_test_configure!(run_in_browser)`
- [x] Added `js_sys::Date` behavior tests (date parsing, NaN filtering, chronological ordering, component extraction) — these genuinely require a browser runtime and cover the sort-by-date feature
- [x] **HTTP mocking evaluation** — four options assessed:
  1. **gloo-net mock facility** — does not exist
  2. **Trait-based HTTP abstraction** — refactor `GitHubClient` to accept `impl HttpClient`; inject a mock in tests. Clean but invasive. Recommended if HTTP tests become a priority.
  3. **Local mock HTTP server** — run localhost during `wasm-pack test --headless --chrome`; point client at it via overridable base URL. No Rust refactoring but adds external test infrastructure.
  4. **Service-worker fetch intercept** — too complex for this project's scale.
- **Decision:** HTTP mocking deferred. Response-parsing, status-code dispatch, and base64 logic are well-covered by the 36 native unit tests. The `tests/wasm.rs` tests cover the JS-runtime-specific behavior that can't run natively.
- **Blocker:** `wasm-pack` is not installed in the Nix dev shell. Add to `flake.nix` to enable `wasm-pack test --headless --chrome` locally and in CI.

#### 11c. Component tests (stretch goal)

- [ ] Evaluate Yew's testing utilities for rendering components in isolation
- [ ] Test key flows: auth redirect when no token, editor loading states, view mode toggling
- [ ] **TODO:** Yew component testing is limited — decide if the effort is justified vs. relying on unit tests + manual verification

#### 11d. CI integration

- [ ] Add a `test` job to the GitHub Actions workflow (or create a new workflow)
- [ ] Run `cargo test` (native unit tests) and `wasm-pack test --headless --chrome` (WASM integration tests)
- [ ] Run on push to `editor/**` branches alongside the existing zola build check
- [ ] Add a `Makefile` target: `make test` that runs both test suites locally

### Phase 12: Migrate Reads to GitHub GraphQL API ✓

**Goal:** Migrate read operations to GraphQL for efficiency; keep mutations and anonymous reads as REST.

**Motivation:** GraphQL returns text file content directly (no base64 decode), has no 1000-entry directory cap, and returns exactly the fields needed. GitHub's GraphQL API requires authentication even for public repos, so anonymous reads (Phase 13) fall back to REST.

**Scope:** Read methods in `services/github.rs` dispatch to GraphQL when authenticated, REST when anonymous. Mutations stay as REST (no benefit from `createCommitOnBranch` complexity). `compare_branches` stays as REST (no GraphQL equivalent for patches).

#### 12a. Core infrastructure ✓

- [x] Add `GRAPHQL_URL` constant (`https://api.github.com/graphql`)
- [x] Add `graphql<T>()` helper method — POSTs query + variables, parses `GraphQLResponse<T>`, surfaces `errors[]` as `Err(String)`, returns `data`
- [x] Add `GraphQLResponse<T>`, `GraphQLError` types in `models/github.rs`
- [x] Both `get_file` paths (GraphQL + REST) now return decoded text — `decode_github_content` made private, component calls removed

#### 12b. Query migrations ✓

| REST method | GraphQL replacement | Status |
|---|---|---|
| `list_contents` | `repository.object(expression) { ... on Tree { entries { name, type, oid, object { ... on Blob { byteSize } } } } }` | ✓ GraphQL when auth'd, REST fallback for anon |
| `get_file` | `repository.object(expression) { ... on Blob { text, oid, byteSize } }` | ✓ Returns plain text, no base64 |
| `get_branch_sha` | `repository.ref(qualifiedName) { target { oid } }` | ✓ GraphQL when auth'd |
| `list_editor_branches` | `repository.refs(refPrefix: "refs/heads/editor/") { nodes { name, prefix, target { oid } } }` | ✓ GraphQL when auth'd |
| `compare_branches` | **Keep as REST** | No GraphQL equivalent for patches |
| `get_last_commit_date` | **Keep as REST** | GraphQL batching is a future optimization |
| `get_check_runs` | **Keep as REST** | Deeply nested GraphQL path, REST is cleaner |

#### 12c. Mutation migrations — deferred

Mutations stay as REST. `createCommitOnBranch` requires `expectedHeadOid` (extra round trip per write) and returns commit OIDs instead of blob SHAs (would require editor component changes). The REST mutation API works well and always requires auth.

#### 12d. Model changes ✓

- [x] Added GraphQL response types: `GraphQLResponse<T>`, `GraphQLError`, `GqlTreeData`, `GqlTree`, `GqlTreeEntry`, `GqlEntryObject`, `GqlBlobData`, `GqlBlob`, `GqlRefData`, `GqlRef`, `GqlRefTarget`, `GqlRefsData`, `GqlRefConnection`, `GqlRefNode`
- [x] REST-specific types (`ContentEntry`, `FileContent`, `GitRef`, `CompareResponse`, etc.) unchanged — GraphQL responses mapped to same types internally
- [x] `decode_github_content` made private — `get_file` now returns decoded text from both paths

#### 12e. Batching opportunities ✓ (partial)

- [x] **Editor load:** `get_branch_sha_and_file()` — new GraphQL method that fetches branch SHA and file content in one round-trip; editor mount effect uses it when authenticated with an active branch, returning early if file found (saves 1 round-trip per editor open)
- [x] **Dashboard sort-by-date:** `get_commit_dates_bulk_graphql()` — builds a dynamic query with one aliased `history(path: $pN, first: 1)` field per path, reducing N parallel REST calls to 1 GraphQL round-trip; `get_commit_dates_bulk()` routes to GraphQL when authenticated
- [ ] **Branch selector + CI status:** batch branch list + check suite status per branch (deferred — CI polling makes this awkward)

### Phase 13: Public View Mode ✓

**Goal:** Make all pages publicly browsable without login; gate write operations behind authentication.

The underlying repo (`pop/pop.github.io`) is public, so the GitHub REST API supports unauthenticated read operations. This phase removes the login requirement for browsing, previewing, and viewing content, while keeping save, publish, delete, upload, and branch operations behind authentication.

- [x] Make `GitHubClient` work without auth — `token` field changed to `Option<String>`; added `anonymous()` constructor and `require_token()` guard; `get` helper conditionally adds `Authorization` header; write methods (`create_or_update_file`, `delete_file`, `upload_binary_file`, `create_branch`, `delete_branch`, `merge_branch`) return error if called without token
- [x] Change default route from Login to Dashboard — `/` now routes to Dashboard; Login moved to `/login`; OAuth redirect_uri updated to `{origin}/login`
- [x] Update nav to always render — shows Dashboard link always; shows Logout button when authenticated, Login link when not
- [x] Remove auth redirect from Dashboard — uses `GitHubClient::anonymous()` when no token; hides "+ New Post" button and branch selector when not authenticated; all read functionality (browsing, searching, sorting) works without login
- [x] Remove auth redirect from Preview — uses anonymous client when no token; fully functional without login
- [x] Remove auth redirect from Editor — uses anonymous client for file loading; when not authenticated: hides Save/Delete/Upload/Publish UI, shows "Login to save" link; textarea editing still works locally
- [ ] Verify: unauthenticated browsing, preview, and editor viewing work end-to-end

### Phase 14: Image Resolution in Preview ✓

**Goal:** Co-located images in markdown posts render correctly in the editor preview pane and standalone Preview page.

**Problem:** Relative image references like `![alt](photo.jpg)` in markdown produce `<img src="photo.jpg">` in the rendered HTML. On the page `/edit/content/posts/my-post/index.md`, the browser resolves this to `/edit/content/posts/my-post/photo.jpg`. Cloudflare Pages serves `index.html` for all paths (SPA fallback), so the request returns 200 with HTML rather than the image.

**Solution:** After rendering markdown to HTML, fetch each relative image from the GitHub API as binary and substitute its `src` with a `data:` URL before embedding into the DOM.

- [x] Add `get_file_bytes(path, branch) -> Result<Vec<u8>, String>` to `GitHubClient` in `services/github.rs` — uses the REST Contents API, parses `FileContent` JSON, strips whitespace from base64, decodes to raw bytes
- [x] Add `post_dir(path: &str) -> &str` to `models/post.rs` — extracts parent directory from a repo file path; replaces the private `parent_dir` function in `editor.rs` (3 call sites updated to import from `models::post`)
- [x] Add `extract_relative_image_srcs(html) -> Vec<String>` to `models/post.rs` — scans `<img` tags for `src=` values not starting with `http`, `//`, `/`, or `data:`; simple string scan, no regex dependency
- [x] Add `replace_image_srcs(html, replacements: &HashMap<String, String>) -> String` to `models/post.rs` — character-accurate in-place replacement of `src` attribute values within `<img` tags
- [x] Add `mime_type_for(path: &str) -> &'static str` to `models/post.rs` — maps `.png`, `.jpg`/`.jpeg`, `.gif`, `.webp`, `.svg` extensions; falls back to `application/octet-stream`
- [x] Add `bytes_to_data_url(bytes: &[u8], path: &str) -> String` to `models/post.rs` — encodes bytes as base64 and produces `data:{mime};base64,{encoded}`
- [x] Add `resolve_images_in_html(html, post_path, branch) -> String` to `GitHubClient` in `services/github.rs` — extracts relative srcs, computes repo paths as `{post_dir}/{src}`, fetches all in parallel via `futures::future::join_all`, replaces successful fetches with data URLs (404s/errors retain original src)
- [x] Update `components/preview.rs` — add `rendered_html` state; in the content-loading async block, call `render_markdown` then `resolve_images_in_html` (always from `source` branch) and store the result; render uses `rendered_html` state; syntax-highlighting effect depends on `rendered_html` instead of `content`
- [x] Update `components/editor.rs` — extend the debounced preview render effect to clone `auth.token`, `props.path`, and `auth.active_branch`, create a `GitHubClient`, and call `resolve_images_in_html` with the active branch (falling back to `source`) before setting `rendered_html`

### Phase 15: Draft/Published Status Icons in Dashboard

**Goal:** Show per-file status icons next to `.md` files in the dashboard file list so users can tell at a glance whether a post is a draft or published.

**Status logic:**
- `draft = true` in TOML frontmatter → 🌱 (draft)
- Frontmatter present with `draft = false` or no `draft` key → 📰 (published)
- No frontmatter → no icon (not a blog post)

**Implementation:**
- [x] Add `PostStatus` enum (`Draft`, `Published`, `NoFrontmatter`) and `detect_post_status(content: &str) -> PostStatus` helper to `components/dashboard.rs`; reuses existing `extract_frontmatter` + `parse_frontmatter` from `models/post.rs`
- [x] Add `post_statuses: use_state(HashMap<String, PostStatus>)` to dashboard component state
- [x] Extend `CachedListing` struct with `post_statuses: Option<HashMap<String, String>>` (serialized as `"draft"`/`"published"`/`"none"`); add `get_cached_post_statuses` and `set_cached_post_statuses` helpers
- [x] Add `use_effect_with(entries)` that: clears stale statuses on navigation, checks cache, then fetches content for each `.md` file via `client.get_file(path, branch)`, parses draft status, caches and sets result
- [x] Update `render_entry` signature to accept `post_statuses: &HashMap<String, PostStatus>` and render 🌱/📰 emoji span before the filename for `.md` files
- [x] Update call site to pass `&*post_statuses`

### Phase 16: Bug Fix — Branch Deselection on Delete

**Goal:** When a branch is deleted (via Publish or Discard in the editor), the dashboard should immediately deselect it and refresh content from `source`, rather than leaving a stale broken state.

**Problem:** After deletion the sessionStorage key `editor_branch` is cleared, but the dashboard's `active_branch` state is not refreshed in the same render cycle. The user sees "fail to load" errors and must manually click "View source" and force-reload.

**Implementation:**
- [x] Investigate `on_confirm_publish` and `on_discard` callbacks in `components/dashboard.rs` — confirmed `force_refresh.set(N+1)` fires before parent propagates `active_branch = None`, causing the effect to run with the deleted branch
- [x] Branch selector panel already closed via `show_branches.set(false)` before the async block
- [x] Removed `force_refresh.set(*force_refresh + 1)` from both handlers — the `active_branch_opt` dependency in the content-loading effect naturally triggers a re-fetch when the parent propagates `None`; `invalidate_all_caches()` ensures fresh data from source
- [x] Verify fix: deleting a branch shows source-branch content with no errors

### Phase 17: Bug Fix — Post Status Filter (All / Draft / Published)

**Goal:** Add filter buttons to the dashboard so users can view All posts, only Drafts, or only Published posts. The existing search bar and sort options remain; this adds a third control group.

**Implementation:**
- [ ] Add `StatusFilter` enum (`All`, `Draft`, `Published`) and `status_filter: use_state(StatusFilter::All)` to dashboard component state in `components/dashboard.rs`
- [ ] Extend the `display_entries` filtering block (approx line 970) to skip `.md` files whose `PostStatus` does not match the selected `StatusFilter`; directories and non-`.md` files always pass through
- [ ] Add a "Filter:" button group (`All` / `Draft` / `Published`) to the `.filter-sort-bar` section in the dashboard render (approx line 1113), styled consistently with the existing sort toggle buttons
- [ ] Add CSS for the new filter buttons (`.status-filter-bar`, `.status-btn`, `.status-btn.active`) in `styles/main.css`, matching the look of `.sort-btn`
- [ ] Reset `status_filter` to `All` on directory navigation (alongside existing `filter_text` reset in `on_navigate` etc.)
- [ ] Verify fix: switching to "Draft" shows only 🌱 files; "Published" shows only 📰 files; "All" restores full list

### Phase 18: Bug Fix — Directory Icon

**Goal:** Display the 📂 emoji for directory entries instead of the current `▸` (U+25B8) triangle.

**Implementation:**
- [ ] In `components/dashboard.rs` `render_entry` (approx line 1345), replace `"\u{25B8}"` with `"📂"`
- [ ] Verify fix: directory rows in the dashboard show 📂 and file rows continue to show `·`

### Phase 19: Bug Fix — Template Tags

**Goal:** Include a `taxonomies.tags` line in the new-post TOML frontmatter template so users have a ready-made example they can uncomment.

**Implementation:**
- [ ] In `components/editor.rs` `generate_template` function (approx line 843), extend the template string to include a commented-out tags line after `draft = true`:
  ```toml
  # taxonomies.tags = ["comics", "games", "backlog", "movies", "tv", "whats-good"]
  ```
- [ ] Verify fix: creating a new post shows the tags comment in the frontmatter

### Phase 20: Feature — Editor Formatting Toolbar

**Goal:** Add five Markdown formatting buttons to the editor toolbar: Bold, Italic, Strikethrough, Inline code, Code block. Each wraps the selected text (or inserts placeholder syntax at the cursor if nothing is selected).

**Implementation:**
- [ ] Add a shared `apply_format` helper in `components/editor.rs` that:
  - Casts `textarea_ref` to `HtmlTextAreaElement`
  - Reads `selection_start()` / `selection_end()` (char positions)
  - Converts to byte offsets via `char_pos_to_byte_offset`
  - Builds new string: `prefix + selected_or_placeholder + suffix`
  - Updates `content` state and calls `on_content_change` to mark dirty
  - Restores focus to the textarea after update
- [ ] Create five `Callback<MouseEvent>` closures (or a single parameterised factory) using `apply_format` for:
  - **Bold** — wrap with `**` / `**`; placeholder `bold text`
  - **Italic** — wrap with `*` / `*`; placeholder `italic text`
  - **Strikethrough** — wrap with `~~` / `~~`; placeholder `strikethrough text`
  - **Inline code** — wrap with `` ` `` / `` ` ``; placeholder `code`
  - **Code block** — wrap with ```` ```\n ```` / ```` \n``` ````; placeholder `code`
- [ ] Add a `.format-buttons` `<div>` to the editor toolbar (approx line 685), rendered unconditionally (formatting is always available regardless of auth state), containing the five buttons
- [ ] Add CSS for `.format-buttons` and `.format-btn` in `styles/main.css`: compact, visually distinct from action buttons (e.g. monospace label, subtle border, smaller font)
- [ ] Verify fix: selecting text and clicking each button wraps it in the correct markdown; clicking with no selection inserts the syntax with a placeholder

---

### Phase 21: Bug — Enable Spell-Check in Editor Textarea

**Goal:** Turn on browser spell-check in the editor textarea so users get red-underline suggestions while writing.

**Implementation:**
- [ ] In `src/components/editor.rs` (approx line 754), change `spellcheck="false"` to `spellcheck="true"` on the `<textarea>` element
- [ ] Verify: typing a misspelled word in the editor shows a squiggly underline

### Phase 22: Bug Fix — File Deletion Cache & Double-Delete

**Goal:** Fix two related bugs in the file deletion flow.

#### Bug 1: Deleted files still appear in cache after deletion

**Root cause:** `invalidate_all_caches()` in `components/dashboard.rs` (~line 161) only clears keys with the `dir_cache_` prefix. The global all-files index cache (`all_files_index` / `all_files_index_{branch}`) is not cleared. As a result, deleted files can reappear when the 5-minute TTL on directory caches is still active.

**Fix:** Extend the key-scan loop in `invalidate_all_caches()` to also remove keys starting with `all_files_index`.

- [ ] In `components/dashboard.rs` `invalidate_all_caches()` (approx line 161), update the `starts_with` check to also match `"all_files_index"` so both cache families are invalidated together
- [ ] Verify: deleting a file and navigating back to the same directory does not show the deleted file

#### Bug 2: Deleting a file twice results in a 404 error

**Root cause:** When a file is already deleted, calling `delete_file()` with its old SHA returns 404 from the GitHub Contents API. The current match in `services/github.rs` (~line 549) only handles 200 and 401; everything else becomes a generic "Failed to delete file: 404" error.

**Fix:** Add an explicit `404` arm to the response match in `delete_file()` returning a clear user-facing error message.

- [ ] In `services/github.rs` `delete_file()` (approx line 549), add `404 => Err("File not found — it may have already been deleted".into())` before the catch-all arm
- [ ] Verify: attempting to delete a previously-deleted file shows a meaningful error instead of "Failed to delete file: 404"

### Phase 23: Feature — CI-Aware Publish Button Colors and Warning Modal

**Goal:** Color the Publish button based on CI state and show a modal warning when CI has not yet passed, rather than silently disabling it.

**Current behavior:** The button is always green (`#28a745`). It is disabled when `ci_blocks_publish = true` (Pending or Failure states). There is no visual distinction between pending/failure, and no path to publish despite pending CI.

**Desired behavior:**
- **Yellow (amber):** CI is pending or no CI runs detected yet — button is enabled, but clicking shows a modal warning ("CI has not passed yet — publish anyway?")
- **Green:** CI passed — normal publish flow
- **Red (disabled):** CI failed — button cannot be clicked

**Implementation:**
- [x] In `components/dashboard.rs`:
  - Added `show_ci_warning_modal: use_state(|| false)` state
  - Changed `ci_blocks_publish` to only block on `CiState::Failure(_)`
  - Updated Publish button's `onclick`: if `CiState::Pending | CiState::None`, sets `show_ci_warning_modal(true)` instead of starting publish; otherwise proceeds normally
  - Added `on_ci_warning_publish` callback (proceeds with diff/publish flow, closes modal first)
  - Added `on_ci_warning_cancel` callback (closes modal)
  - Added conditional CI warning modal with "Publish anyway" + "Cancel" buttons
  - Added dynamic CSS class to Publish button: `publish-btn pending` for Pending/None, `publish-btn failure` for Failure, default (green) for Success
- [x] In `styles/main.css`:
  - Added `.publish-btn.pending { background: #d97706; }` (amber) with hover state
  - Added `.publish-btn.failure { background: #dc3545; }` (red) with hover state
  - Existing modal overlay styles reused (`.modal-overlay`, `.modal`, `.modal-actions` already present)

---

### Phase 24: Bug Fix — Split Editor Independent Scrolling

**Goal:** In Split view mode, the editor textarea and preview pane should scroll independently; currently they share a single scroll context.

**Root cause:** The `.editor-container` is a flex row, but neither child has a bounded height with independent overflow. The textarea has `min-height: 60vh` (no `overflow-y`), and `.preview-pane` has `overflow-y: auto` but also only `min-height` — so both expand to their content height rather than scrolling within a fixed viewport.

**Implementation:**
- In `styles/main.css`, within the `.editor-container.split` context (or globally for split mode):
  - Set `.editor-container` to `align-items: stretch` and a fixed height (e.g. `height: calc(100vh - 180px)`)
  - Set `.editor-container .editor-textarea { height: 100%; overflow-y: auto; resize: none; }` — removes `min-height`, gives independent scroll
  - Set `.editor-container .preview-pane { height: 100%; overflow-y: auto; }` — existing `overflow-y: auto` may suffice once height is bounded
- Scope these rules to `.editor-container.split` to avoid affecting Edit-only or Preview-only modes

---

### Phase 25: Feature — Mobile Split Editor Stacking

**Goal:** On small screens, Split mode should stack the editor and preview vertically (one above the other) instead of side-by-side.

**Current state:** There are no `@media` queries in `styles/main.css`. The split layout uses `display: flex` with the default `flex-direction: row`, making both panes narrow on mobile.

**Implementation:**
- In `styles/main.css`, add a responsive breakpoint (≤768px):
  ```css
  @media (max-width: 768px) {
    .editor-container.split {
      flex-direction: column;
    }
    .editor-container.split .editor-textarea,
    .editor-container.split .preview-pane {
      width: 100%;
      min-height: 50vh;
    }
    .editor-page-wide {
      max-width: 100%;
      padding: 0 0.5rem;
    }
  }
  ```
- No Rust changes needed — CSS-only fix

### Phase 26: Bug Fix — Move Sync Button to Branch Badge with Consistent Styling [064aaa] ✓

**Goal:** The "Sync from source" button belongs in the active branch badge on the
dashboard, not in the editor toolbar. It also has no CSS styling.

**Implementation:**

- [x] Removed `syncing` state, `on_sync` callback, and sync button from `editor.rs`
- [x] Added `syncing` state, `on_sync` callback, and "Sync from source" button to
  `dashboard.rs` inside `.branch-badge-actions` (between Publish and Discard)
- [x] Added `.sync-btn` CSS to `styles/main.css` (neutral grey outlined style)

### Phase 27: Bug Fix — Dashboard Filter Skips Folder-Posts [c5d99b] ✓

**Goal:** The Draft / Published status filter should apply to folder-posts
(directories containing a single `index.md`) using the already-computed
`folder_md_statuses` data.

**Implementation:**

- [x] Replaced single-condition filter block with a branched approach:
  - `.md` files → check `post_statuses`
  - `dir` entries with a known `folder_md_statuses` entry → check that status
  - Everything else (section dirs, media dirs, etc.) → always pass through

### Phase 28: Feature — Trailing `/` on Folder-Posts in Dashboard [e9f800] ✓

**Goal:** Append a trailing `/` to the display name of directory entries in the
dashboard list so folder-posts are visually distinguishable from standalone `.md` files
at a glance.

**Implementation:**

- [x] Changed `{&entry.name}` to `{ format!("{}{}", &entry.name, if is_dir { "/" } else { "" }) }`
  in `render_entry` in `dashboard.rs`

---

## Open Questions

- ~~**Hosting:** Where to deploy the built WASM app.~~ **Resolved:** Cloudflare Pages at `editor.elijah.run`, managed via OpenTofu.
- **Conflict resolution:** If `source` changes while editing, a merge may
  conflict. Initial approach: show an error and suggest the user resolve
  manually on GitHub.
- ~~**Worker deployment:** Need to set up wrangler config and GitHub OAuth
  App credentials.~~ **Resolved:** Deployed via OpenTofu (`cloudflare_worker` + `cloudflare_worker_version` + `cloudflare_workers_deployment`). Secrets configured as `secret_text` bindings in the worker version. Infrastructure config in `editor/infra/`.

---

## Session Log

### Session 22 (2026-02-23) — Ticket cc5ba0: Phase 11b WASM test infrastructure

- Added `wasm-bindgen-test = "0.3"` to `[dev-dependencies]` in `Cargo.toml`
- Created `tests/wasm.rs` gated with `#![cfg(target_arch = "wasm32")]` — excluded from native `cargo test`, runs only under `wasm-pack test --headless --chrome`
- Added 5 `js_sys::Date` tests covering: valid ISO-8601 parse, invalid string → NaN, empty string → NaN, chronological ordering, date component extraction
- Added 10 missing pure-function tests to `models/post.rs`: `extract_relative_image_srcs` edge cases (data URL, root-relative, multiple), `replace_image_srcs` (substitution, unmatched, no-op), `mime_type_for` (known extensions, fallback), `bytes_to_data_url` (data URL format, MIME selection)
- Native test suite now covers 36 tests; all pass
- HTTP mocking evaluation documented in `tests/wasm.rs` header and PLANNING.md (gloo-net has no mock; trait abstraction is the recommended path; deferred)
- Blocker noted: `wasm-pack` not in Nix dev shell; must be added to `flake.nix` before WASM tests can run

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

### Session 7 (2026-02-14) — Robustness & UX improvements
- Implemented 6 robustness/UX items (TODO items 4-9 from review)
- **Session expiry handling (all components):** Added `Rc<dyn Fn(String)>` error wrapper in editor that checks for "Unauthorized" and clears auth token; added same 401 detection in dashboard and preview load effects; all `GitHubClient` methods already return "Unauthorized" on 401
- **Externally deleted branch handling:** Editor load effect now calls `get_branch_sha` to verify stored branch exists before loading file; clears sessionStorage and falls back to source branch on 404
- **Image conflict handling:** `upload_binary_file` now accepts `sha: Option<&str>` parameter; editor checks for existing file at upload path via `get_file` and passes SHA to overwrite
- **Keyboard shortcuts:** Document-level `keydown` listener via `wasm_bindgen::closure::Closure`; Ctrl+S / Cmd+S clicks save button programmatically via `NodeRef`; cleanup removes listener on unmount
- **Unsaved changes warning:** `use_mut_ref` tracks dirty state (content != original or is_new); `beforeunload` listener via Closure checks flag and calls `prevent_default` + `set_return_value`
- **Drag-and-drop image upload:** Shared `Callback<web_sys::File>` used by both file input and drag-drop; `ondragover`/`ondragleave`/`ondrop` on editor container; `dragging` state adds `.drag-over` CSS class with dashed purple outline
- Added web-sys features: `BeforeUnloadEvent`, `DataTransfer`, `DragEvent`, `EventTarget`, `KeyboardEvent`
- CSS: `.drag-over` class with dashed outline and subtle background tint; transition on editor container
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only `ref_name` dead-code warning remaining)

### Session 8 (2026-02-14) — Final TODO items
- Implemented 5 remaining TODO items (pagination, debounce, syntax highlighting, image validation, diff preview)
- **Pagination (Trees API fallback):** `list_contents` detects when Contents API returns 1000 entries (its hard cap) and falls back to `list_contents_via_tree` using the Git Trees API with `recursive=1`; filters tree entries to direct children of requested path; converts `TreeEntry` to `ContentEntry` for seamless dashboard display. Added `TreeResponse`/`TreeEntry` models in `models/github.rs`.
- **Debounced markdown rendering:** Added `rendered_html` state and `render_gen` generation counter. `use_effect_with` on `(content, show_preview)` spawns an async task that sleeps 200ms then checks if its generation is still current before rendering. Preview pane displays `rendered_html` instead of calling `render_markdown` directly. Shows "Rendering..." placeholder while waiting.
- **Syntax highlighting:** Added highlight.js v11.9 CSS + JS via CDN in `index.html`. Editor has `use_effect_with` on `rendered_html` that calls `hljs.highlightElement` on all unhighlighted `<code>` blocks via `js_sys::eval`. Preview component has the same highlighting effect after content loads.
- **Image size/type validation:** Added validation at the start of `upload_image` callback before any async work. Checks MIME type against allowed list (png, jpeg, gif, webp, svg+xml) and file size (10 MB max). Shows error message with actual file size on rejection.
- **Diff preview before publishing:** Added `compare_branches` method to `GitHubClient` using GitHub Compare API (`GET /repos/{owner}/{repo}/compare/{base}...{head}`). Added `CompareResponse`/`DiffFile` models. "Publish" button now fetches branch comparison and shows a diff panel with: summary (files changed, additions, deletions), per-file patches with +/- coloring, status badges (A/M/D/R). "Confirm Publish" and "Cancel" buttons in diff panel. Full diff CSS: `.diff-panel`, `.diff-file-header`, `.diff-patch`, `.diff-line-add/del/hunk/ctx`.
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — only `ref_name` and `truncated` dead-code warnings)

### Session 9 (2026-02-17) — Phases 8c, 8d, 10a, 10b
- Built Phase 8c: directory listing cache
- `CachedListing` struct with entries + timestamp in `dashboard.rs`; `get_cached_listing`/`set_cached_listing` use sessionStorage with `dir_cache_{path}` keys and 5-minute TTL; `invalidate_cache` (pub) and `invalidate_all_caches` (pub) exported for use by editor
- Dashboard fetch effect checks cache before API call; `force_refresh` counter in effect deps bypasses cache when incremented
- Refresh button (↻) added to dashboard header alongside branch toggle and new post buttons
- Editor calls `invalidate_cache(parent_dir)` on save/delete/publish; `invalidate_all_caches` on discard
- Added `Serialize` derive to `ContentEntry` for sessionStorage serialization
- Built Phase 8d: branch selector
- `list_editor_branches` method on `GitHubClient` using `GET /repos/{owner}/{repo}/git/matching-refs/heads/editor/`
- `list_contents` now accepts `Option<&str>` branch parameter (was hardcoded to `source`); dashboard passes active branch
- Branch selector UI: toggle button, branch list panel with clickable items, active branch badge with clear button, "View source" to deselect
- Switching branches invalidates all caches and forces refresh
- Built Phase 10a: GitHub Actions workflow
- `.github/workflows/editor-check.yml`: triggers on `push` to `editor/**` branches; checks out with LFS + submodules; installs Zola 0.19.2 via `taiki-e/install-action@v2`; runs `zola build`
- Built Phase 10b: CI status in editor UI
- `CheckRunsResponse`/`CheckRun` models in `models/github.rs`; `get_check_runs` method on `GitHubClient`
- `CiState` enum (Pending, Success, Failure with URL, None); `fetch_ci_status` async helper aggregates check runs into single state
- CI indicator in publish bar: hourglass for pending, checkmark for success, X with link for failure; hidden when no check runs
- Publish button disabled when CI is pending or failed (`ci_blocks_publish` flag)
- Polls every 15 seconds via `setInterval`; stops when terminal state reached; interval cleared on component unmount
- CSS: `.dashboard-actions`, `.refresh-btn`, `.branch-toggle-btn`, `.active-branch-badge`, `.branch-list`, `.branch-item`, `.ci-status`, `.ci-pending/success/failure`
- Compiles clean (`cargo check --target wasm32-unknown-unknown` — dead-code warnings for `truncated`, `total_count`, `id`, `name` fields in deserialized structs)
- Deployed to Cloudflare Pages (production)

### Session 10 (2026-02-17) — Phases 8a, 8b
- Built Phase 8a: search/filter for dashboard content list
- Filter input (`.filter-input`) with case-insensitive substring matching; clear button (×) to reset; filter resets automatically on directory navigation
- "No entries match the filter." empty state when filter excludes everything
- Built Phase 8b: sort options (Alphabetical / Last Modified)
- `SortMode` enum with two toggle buttons ("A–Z" / "Recent") in `.sort-bar`
- Lazy date fetching: commit dates only fetched via `get_commit_dates_bulk` (parallel `futures::join_all`) when user clicks "Recent"
- `CachedListing` extended with `commit_dates: Option<HashMap<String, f64>>` for caching dates alongside entries in sessionStorage
- `get_last_commit_date` and `get_commit_dates_bulk` methods added to `GitHubClient`; `CommitInfo`/`CommitDetail`/`CommitAuthor` models added
- `format_date(epoch_ms)` helper renders dates as `YYYY-MM-DD` using `js_sys::Date`
- CSS: `.filter-sort-bar`, `.filter-bar`, `.filter-input`, `.clear-filter-btn`, `.sort-bar`, `.sort-btn`, `.loading-dates`, `.entry-date`
- Compiles clean (same dead-code warnings as before)

### Session 11 (2026-02-17) — Phase 9
- Built Phase 9: preview enhancements (frontmatter display)
- Added `toml = "0.8"` crate to `Cargo.toml`
- `extract_frontmatter` in `models/post.rs`: extracts raw TOML between `+++` delimiters; reuses same delimiter-finding logic as `strip_frontmatter`
- `parse_frontmatter` in `models/post.rs`: parses TOML into `Vec<(String, String)>` key-value pairs via `toml::from_str::<toml::Table>`; nested tables flattened with dot notation (e.g. `taxonomies.tags`); graceful degradation on parse failure (returns empty vec)
- `format_toml_value` helper: strings unquoted, arrays comma-separated, booleans/numbers/datetimes as-is
- `flatten_toml` helper: recursively walks `toml::Table`, building dotted key paths for nested tables
- Editor (`components/editor.rs`): added `frontmatter_fields` state; updated debounced preview render effect to call `parse_frontmatter` alongside `render_markdown`; renders `.frontmatter-table` above markdown in preview pane
- Preview (`components/preview.rs`): added `frontmatter_fields` state; parsed after file load; same table rendering before markdown body
- CSS: `.frontmatter-table` (muted `#f6f8fa` background, `0.85rem` font, full width, collapsed borders), `.fm-key` (bold, 120px width), `.fm-value` (word-break)
- Compiles clean (same dead-code warnings as before)

### Session 12 (2026-02-17) — Phase 13
- Built Phase 13: public view mode (unauthenticated browsing)
- `GitHubClient` token changed to `Option<String>`; added `anonymous()` constructor and `require_token()` guard for write methods; `get` helper conditionally adds `Authorization` header
- Routes: default route (`/`) changed from Login to Dashboard; Login moved to `/login`; OAuth redirect_uri updated to `{origin}/login`
- Nav: always renders with Dashboard link; shows Logout when authenticated, Login link when not
- Dashboard: removed auth redirect; uses `GitHubClient::anonymous()` when no token; hides "+ New Post" button and branch selector when unauthenticated; all read ops (browsing, search, sort) work without login
- Preview: removed auth redirect; uses anonymous client; fully functional without login
- Editor: removed auth redirect; uses anonymous client for file loading; hides Save/Delete/Upload Image/Publish UI when unauthenticated; shows "Login to save" link; textarea editing works locally
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)

### Session 13 (2026-02-17) — Phase 12
- Built Phase 12: migrate reads to GraphQL API (hybrid approach)
- Added `GRAPHQL_URL` constant and `graphql<T>()` helper method to `GitHubClient` — POSTs query + variables to `/graphql`, parses `GraphQLResponse<T>` envelope, surfaces `errors[]` as `Err(String)`
- Added GraphQL response types in `models/github.rs`: `GraphQLResponse<T>`, `GraphQLError`, `GqlTreeData`/`GqlTree`/`GqlTreeEntry`/`GqlEntryObject` (Tree queries), `GqlBlobData`/`GqlBlob` (Blob queries), `GqlRefData`/`GqlRef`/`GqlRefTarget` (ref queries), `GqlRefsData`/`GqlRefConnection`/`GqlRefNode` (refs listing)
- Migrated `list_contents` — GraphQL Tree query when authenticated (no 1000-entry cap, replaces both Contents API and Trees API fallback), REST fallback for anonymous
- Migrated `get_file` — GraphQL `Blob.text` returns plaintext directly; REST path now decodes base64 internally; both paths return decoded text in `FileContent.content`
- Migrated `get_branch_sha` — GraphQL `ref.target.oid` when authenticated, REST fallback
- Migrated `list_editor_branches` — GraphQL `refs(refPrefix)` when authenticated, REST fallback
- Removed `decode_github_content` calls from `editor.rs` and `preview.rs` — `get_file` now returns decoded text; `decode_github_content` made private (used only by REST `get_file_rest`)
- Kept as REST: all mutations, `compare_branches`, `get_check_runs`, `get_last_commit_date`/`get_commit_dates_bulk`
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)

### Session 14 (2026-02-19) — Phase 14
- Built Phase 14: image resolution in preview
- Root cause: relative image `src` values in rendered markdown produce requests to `/edit/{path}`, which Cloudflare Pages serves as `index.html` (SPA fallback) rather than the actual image binary
- `get_file_bytes(path, branch) -> Result<Vec<u8>, String>` added to `GitHubClient` in `services/github.rs` — REST Contents API, parses `FileContent` JSON, decodes base64 to raw bytes
- Image utilities added to `models/post.rs`: `post_dir` (parent directory extraction, replaces private `parent_dir` in `editor.rs`), `extract_relative_image_srcs` (scans `<img` tags for relative `src=` values), `replace_image_srcs` (character-accurate attribute substitution), `mime_type_for` (extension-to-MIME mapping), `bytes_to_data_url` (base64 `data:` URL construction)
- `resolve_images_in_html(html, post_path, branch) -> String` added to `GitHubClient` — orchestrates parallel image fetches via `futures::future::join_all`, converts successful results to `data:` URLs, leaves unresolvable images at their original src
- `components/preview.rs`: added `rendered_html` state; content-loading async block now calls `render_markdown` → `resolve_images_in_html` (source branch) → `rendered_html.set`; render uses `rendered_html` state; syntax-highlighting effect moved to depend on `rendered_html`
- `components/editor.rs`: debounced render effect extended to create `GitHubClient` from token, call `resolve_images_in_html` with active branch (or source), store resolved HTML; `parent_dir` references replaced with `post_dir` imported from `models::post`

### Session 21 (2026-02-23) — Ticket f2abcd: Fix absent-draft-key icon for standalone .md files

- `detect_post_status()` in `dashboard.rs` was returning `PostStatus::NoFrontmatter` for `.md` files with valid frontmatter but no explicit `draft` key (the `None` arm in the match was wrong).
- Zola's default when `draft` is absent is `draft = false` (published), so absent key → Published is correct.
- Fixed by collapsing `Some(_) => Published` and `None => NoFrontmatter` into a single `_ => Published` arm; the early-return at the top of `detect_post_status()` already handles the true no-frontmatter case.
- This is the standalone-file counterpart of the folder fix applied in session 20 (ticket 469284).
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)

### Session 20 (2026-02-23) — Tickets 469284, a066f2, 1cc1d4, 2fd5c2

#### Ticket 469284: Folder with draft=false/.md shows no icon

- Root cause: folder status detection used `detect_post_status()` which returns `NoFrontmatter` for files with frontmatter but no `draft` key (after 623c82 fix). For folder icons the rule should be: any frontmatter + not `draft=true` → Published icon.
- Fixed in Phase 2 of folder status detection: inline logic that treats absent `draft` key as Published (not NoFrontmatter), so folders containing an `index.md` with frontmatter and no draft key now correctly display 📰.

#### Ticket a066f2: Directories with sub-directories always show folder icon

- Section-level directories (e.g. `content/blog/`) can contain both sub-directories and an `index.md`. These should always show 📂, not a post icon.
- Fixed in Phase 2 filter_map: early-return `None` if any child entry is a `dir`. This prevents section directories from being mistaken for leaf post directories.

#### Tickets 1cc1d4 / 2fd5c2: Branch last-activity display (deferred)

- Both tickets requested showing commit date/count per branch in the branch selector.
- Deferred: branch names already encode the creation date (`editor/YYYY-MM-DD-slug`), and the extra API calls (one per branch) are not worth the cost.
- Closed both tickets as done.

### Session 19 (2026-02-23) — Tickets 279208, 623c82

#### Ticket 623c82: Fix missing-draft-key icon bug

- `detect_post_status()` in `dashboard.rs` was treating "no draft key" the same as "draft = false" — both returned `Published` (📰)
- Fixed by switching from `any(k == "draft" && v == "true")` to `find(k == "draft").map(v.as_str())` with three arms: `Some("true")` → Draft, `Some(_)` → Published, `None` → NoFrontmatter (no icon)
- Files with frontmatter but no `draft` key now show no icon (consistent with Zola's implicit-published behaviour but without claiming a Published status we can't confirm)

#### Ticket 279208: Sync action in editor

- Built: "Sync from source" button in the editor toolbar
- Added `syncing: use_state(|| false)` to track async operation
- Added `on_sync` callback: calls `merge_branch(head=DEFAULT_BRANCH, base=editor_branch, ...)` to pull source changes into the current editor branch
- Button only appears when `auth.active_branch.is_some()` (i.e. an editor branch exists)
- All other toolbar buttons (Save, Delete, Upload Image) disabled while syncing
- Shows "Syncing…" label during operation, "Synced from source" success message
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)

### Session 18 (2026-02-23) — Ticket system migration: bd → nbd

- Migrated all 33 bd tickets from `.beads/issues.jsonl` into nbd
- Priority mapping applied: bd 1→9, bd 2→7, bd 3→5, bd 4→3
- Dependencies wired up between nbd tickets (6 tickets with deps)
- 7 closed bd tickets archived in nbd
- 26 open bd tickets are now active nbd todos

### Session 17 (2026-02-21) — Phase 23: CI-Aware Publish Button

- Built Phase 23: CI-aware Publish button colors and warning modal
- `show_ci_warning_modal` state added to dashboard component
- `ci_blocks_publish` now only blocks on `CiState::Failure` (was Pending|Failure)
- Publish button onclick: if CI is Pending or None, shows warning modal instead of proceeding; if Success, proceeds normally; if Failure, button is disabled
- Added `on_ci_warning_publish` and `on_ci_warning_cancel` callbacks
- CI warning modal: "CI has not passed yet" heading, explanatory text, "Cancel" + "Publish anyway" buttons
- Dynamic Publish button class: `publish-btn pending` (amber `#d97706`) for Pending/None, `publish-btn failure` (red `#dc3545`) for Failure, default green for Success
- CSS: `.publish-btn.pending` and `.publish-btn.failure` overrides with hover states added to `styles/main.css`
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)

### Session 16 (2026-02-21) — Dashboard Icon Improvements

- **Media icon (🖼️):** Media files now show 🖼️ in both global search results (was `·`) and regular directory listings (was no icon). `is_media` guard already existed in both code paths; added `else if is_media` branch in `render_entry` status_icon logic and an `if is_media` branch in the search result HTML.
- **Folder single-.md status icon:** Added `folder_md_statuses: use_state(HashMap<String, PostStatus>)` state. New `use_effect_with` effect (same dependency as `post_statuses`) fetches directory children for each folder in the current listing, then fetches the `.md` file for folders with exactly one `.md` child, runs `detect_post_status`, and populates the map. `render_entry` now checks `folder_md_statuses.get(&entry.path)` for directories and shows 🌱/📰 (or 📂 if no single `.md` or status is `NoFrontmatter`). Phase 1 uses the existing listing cache, so redundant API calls are rare.
- BUGS.txt cleared (both items implemented).

### Session 15 (2026-02-19) — Phase 15
- Built Phase 15: draft/published status icons in dashboard
- `PostStatus` enum (`Draft`, `Published`, `NoFrontmatter`) and `detect_post_status(content: &str) -> PostStatus` helper added to `components/dashboard.rs`; reuses `extract_frontmatter` + `parse_frontmatter` from `models/post.rs`
- `post_statuses: Option<HashMap<String, String>>` field added to `CachedListing` (with `#[serde(default)]`); `get_cached_post_statuses` and `set_cached_post_statuses` helpers added alongside existing commit-date cache helpers
- `post_statuses: use_state(HashMap<String, PostStatus>)` state added to dashboard component
- `use_effect_with((path, entries.len()))` effect: clears stale statuses on navigation, checks cache, then fetches content for each `.md` file in parallel via `join_all` + `client.get_file`, parses draft status, caches and sets result
- `render_entry` extended to accept `post_statuses: &HashMap<String, PostStatus>`; renders 🌱 before filename for draft `.md` files, 📰 for published
- Compiles clean (`cargo check --target wasm32-unknown-unknown`)
