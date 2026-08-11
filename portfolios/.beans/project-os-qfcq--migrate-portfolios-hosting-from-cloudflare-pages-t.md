---
# project-os-qfcq
title: Migrate portfolios hosting from Cloudflare Pages to pages.elijah.run zip upload
status: in-progress
type: feature
priority: normal
created_at: 2026-07-24T23:30:08Z
updated_at: 2026-07-27T04:38:08Z
---

Move the Win95 portfolios site off manually-managed Cloudflare Pages/wrangler onto pages.elijah.run, which accepts a <=10MB archive served at /portfolios. Requires making the site work under the /portfolios subpath, producing a tar.zst artifact, and a transparent (serve-not-301) rewrite from the legacy portfolio.elijah.run host.

## Dev env: flake toolchain switched to rustup-file style (like gamez)
- flake.nix now uses pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml (rust-overlay retained), matching gamez/flake.nix pattern; devShell -> devShells.default.
- Added rust-toolchain.toml: channel=stable, targets=[wasm32-unknown-unknown], components rust-src/rust-analyzer/clippy. (Not gamez's nightly+cranelift — that's native-Bevy-only; this is a wasm/trunk web app.)
- Added sccache (RUSTC_WRAPPER, devshell-scoped) and binaryen (wasm-opt); dropped Cloudflare/unused tools: wrangler, worker-build, wasm-pack.
- Validated: devshell derivation instantiates (nix eval drvPath OK). NOTE: rust-toolchain.toml must be git-tracked for nix to see it (staged via git add -N).
- Full 'just package' build still pending on disk space (/ at 99%).

## Dev env: split devshell (lean default + opt-in full)
- Problem: single fat devShell pulled ffmpeg + imagemagick + bun + playwright Chromium (~4.3 GiB unpacked / 1.3 GiB DL) for a routine 'just package', hanging on a 99%-full disk.
- Fix: devShells.default is now lean (rust, trunk, wasm-bindgen-cli, binaryen, sccache, jq, beans, just) = 246 MiB DL / 762 MiB unpacked, no media/browser closures. Heavy asset-prep + e2e tools moved to 'nix develop .#full'.
- Verified via nix build --dry-run inputDerivation: default 100 paths/246MiB vs full 498 paths/4.3GiB.

## Repo work complete — manual cutover remains
All three child tasks done and verified against a real build (portfolios.tar.zst = 1.6 MB, relative paths confirmed). Left in-progress until the live hosting cutover, which is manual/out-of-repo:
1. Upload portfolios.tar.zst to pages.elijah.run (served at /portfolios).
2. Deploy deploy/games-legacy-host.worker.js (or the Terraform ruleset) + bind route games.elijah.run/* + proxied DNS.
3. Decide handling for games.elijah.run/set/ (sibling game) so the blanket rewrite doesn't clobber it.

## Dev env (final): single do-everything default shell
Superseded the earlier lean/full/deploy split (the disk crunch that motivated it is resolved — / now has ~67 GB free). Per user request, devShells.default now carries the full maintenance toolchain: rust(+trunk/wasm-bindgen/binaryen/sccache) for build/package, imagemagick/ffmpeg for asset prep, bun/playwright for e2e, wrangler for the Worker deploy, plus jq/beans/just. shellHook sets RUSTC_WRAPPER=sccache and the PLAYWRIGHT_* vars. Deploy docs updated: 'nix develop -c wrangler deploy -c deploy/wrangler.toml'.

## Migration LIVE
games.elijah.run transparently serves pages.elijah.run/portfolios (worker + service binding + proxied DNS). Fixed bundle uploaded and verified end-to-end. Remaining: commit repo changes (portfolios/ + editor/infra/games-elijah-run.tf).
