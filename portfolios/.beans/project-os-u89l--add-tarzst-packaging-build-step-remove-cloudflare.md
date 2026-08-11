---
# project-os-u89l
title: Add tar.zst packaging build step; remove Cloudflare Pages/wrangler
status: completed
type: task
priority: normal
created_at: 2026-07-24T23:30:21Z
updated_at: 2026-07-25T22:48:49Z
parent: project-os-qfcq
---

Replace the wrangler deploy flow with a build step that emits a tar.zst archive of dist/ for upload to pages.elijah.run.

## Tasks
- [ ] Justfile: add 'package' recipe (clean + build + tar --zstd of dist -> portfolios.tar.zst)
- [ ] Justfile: remove the wrangler 'deploy' recipe
- [ ] Delete wrangler.toml and .wrangler/
- [ ] .gitignore: add portfolios.tar.zst
- [ ] Verify: just package produces portfolios.tar.zst under 10MB

## Notes
- User chose tar.zst only (no zip).
- Justfile: added 'package' recipe (clean + build + ZSTD_CLEVEL=19 tar --zstd of dist -> portfolios.tar.zst); removed wrangler 'deploy' recipe.
- Deleted wrangler.toml and .wrangler/.
- .gitignore: added portfolios.tar.zst.
- [ ] Verify artifact size < 10MB (build running)

## Blocker: build not yet verified
- flake.nix devShell still listed wrangler (Cloudflare) -> now removed as part of this migration; it was also the derivation failing to build.
- Root blocker: / (holds /nix) is 99% full (~3.8GB free). nix develop couldn't realize the devshell (ENOSPC building wrangler-pnpm-deps). Project FS has 69GB, but nix-store writes go to /.
- trunk 0.21.14 IS cached in the store.
- NOT verified: whether public_url = "./" makes Trunk emit relative asset links (./main-*.css etc.) vs root-absolute. This is the one behavioral assumption to confirm with a real build.
- Recommend: free store space (nix-collect-garbage -d) then run 'just package', confirm dist/index.html uses relative paths and artifact < 10MB.

## Summary of Changes
Replaced the wrangler Cloudflare Pages deploy with a tar.zst packaging step.
- Justfile 'package' recipe: clean + trunk build --release + ZSTD_CLEVEL=19 tar --zstd of dist/ -> portfolios.tar.zst.
- Removed wrangler 'deploy' recipe, wrangler.toml, .wrangler/.
- .gitignore: added portfolios.tar.zst.
Verified: real build produced portfolios.tar.zst = 1.6 MB (< 10 MB limit), flat-rooted archive with all 24 site files at ./ (index.html, wasm, css, mp4s, opus, wsz, icons).
