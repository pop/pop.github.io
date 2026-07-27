---
# project-os-h6up
title: Automate portfolios upload to pages.elijah.run
status: completed
type: task
priority: normal
created_at: 2026-07-27T17:26:36Z
updated_at: 2026-07-27T17:29:01Z
---

Add a 'just upload' recipe that PUTs portfolios.tar.zst to pages.elijah.run/api/pages/portfolios using a Bearer API token (PAGES_ELIJAH_RUN_API_TOKEN from .env). Wire it into 'just release'. Bundle format (.tar.zst) is already accepted by the pages Worker; no cf_clearance needed.

## Summary of Changes

- Confirmed the pages Worker natively accepts .tar.zst (UploadKind::Tar(Zstd)); no format change needed.
- Verified /api/* is NOT behind a Cloudflare challenge, so no cf_clearance is required — Bearer token + PUT is sufficient.
- Added `just upload` recipe: PUTs portfolios.tar.zst to /api/pages/portfolios with the PAGES_ELIJAH_RUN_API_TOKEN Bearer token (loaded from .env via `set dotenv-load`), checks for HTTP 200.
- Wired `upload` into `just release` (package deploy-worker upload).
- Updated deploy/README.md to drop the 'manual upload' note.
- Tested end-to-end: upload returned 200 (24 files, 1977941 bytes); `just verify` shows transparent 200 on games.elijah.run.
