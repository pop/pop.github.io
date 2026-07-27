---
# project-os-mq9i
title: Transparent rewrite portfolio.elijah.run -> pages.elijah.run/portfolios
status: completed
type: task
priority: normal
created_at: 2026-07-24T23:30:26Z
updated_at: 2026-07-27T04:38:08Z
parent: project-os-qfcq
---

Legacy host portfolio.elijah.run must transparently SERVE (not 301) the content at pages.elijah.run/portfolios. Cloudflare-proxied.

## Tasks
- [ ] Add deploy/portfolio-redirect.worker.js: Worker that rewrites host to pages.elijah.run and ensures paths live under /portfolios (root -> /portfolios/, pass through existing /portfolios/ asset requests to avoid double-prefix)
- [ ] Document the Cloudflare route binding (portfolio.elijah.run/*) + proxied DNS record
- [ ] Document the declarative Terraform alternative (cloudflare_ruleset: http_request_origin host override + http_request_transform path rewrite)

Note: infra lives outside the repo; this bean only lands the worker script + docs. Actual CF deploy is manual.

## Notes
- Legacy host is games.elijah.run (not portfolio.elijah.run) per user; canonical domain kept as games.elijah.run.
- Added deploy/games-legacy-host.worker.js: rewrites host->pages.elijah.run and prefixes path with /portfolios (flat 1:1 because assets are relative). Serve, not 301.
- Added deploy/README.md: wrangler route/DNS binding + declarative Terraform alternative (cloudflare_ruleset http_request_origin + http_request_transform) + caveat about other paths (e.g. games.elijah.run/set/).
- Infra deploy remains manual/out-of-repo.

## Summary of Changes
Landed the transparent-serve mechanism as code + docs (actual Cloudflare deploy is manual/out-of-repo).
- deploy/games-legacy-host.worker.js: reverse-proxy Worker, host->pages.elijah.run, path prefixed with /portfolios. Flat 1:1 prefix works because the built site uses relative asset paths (verified). Serve, not 301.
- deploy/README.md: wrangler route (games.elijah.run/*) + proxied DNS, the declarative Terraform alternative (cloudflare_ruleset http_request_origin host override + http_request_transform path rewrite), and a caveat about other paths on games.elijah.run (e.g. /set/).
Not done here (manual, by user): upload portfolios.tar.zst to pages.elijah.run, deploy the Worker/ruleset, point DNS.

## Worker deployed (DNS pending)
- wrangler deploy OK: worker 'games-legacy-host', route games.elijah.run/* (zone elijah.run) bound, version 3520ebae-9ec7-4ea7-b1a6-023a96bfb4b1. Authed as homeworkbad@gmail.com (workers/workers_routes/workers_scripts write scopes present).
- BLOCKER for live cutover: games.elijah.run does not resolve — proxied DNS record not yet created (removing the old Pages custom domain removed its record). Apply deploy/dns.tf (terraform) or add AAAA games=100:: proxied in the dashboard.
- Also pending: re-upload the fixed portfolios.tar.zst (icon is_url bug) to pages.elijah.run/portfolios/.

## LIVE — transparent serve working
games.elijah.run now serves the portfolios site transparently (200, no 301) from pages.elijah.run/portfolios.

Key fix: pages.elijah.run is itself a Worker (route pages.elijah.run/* -> script 'pages'). A same-zone Worker->Worker plain fetch() bypasses it and hits a dead origin (522). Solution: Service Binding — wrangler.toml [[services]] binding=PAGES service=pages, and the worker calls env.PAGES.fetch(). 

DNS: proxied AAAA games=100:: created via terraform in editor/infra/games-elijah-run.tf (the only TF root wired to the elijah.run zone). Worker deployed via wrangler (deploy/). Verified: /=200 transparent, fixed bundle served, all assets 200 via flat /portfolios prefix, /set/=404.
