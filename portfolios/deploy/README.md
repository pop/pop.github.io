# Legacy-host rewrite: `games.elijah.run` → `pages.elijah.run/portfolios`

The site is built into `portfolios.tar.zst` (`just package`) and uploaded to
**pages.elijah.run** (`just upload` — `PUT /api/pages/portfolios` with the
`PAGES_ELIJAH_RUN_API_TOKEN` Bearer token from `.env`), which serves it at
**`/portfolios/`**. `just release` does both, then deploys the Worker below.
The legacy domain
**games.elijah.run** must keep working and *serve* that content transparently —
no `301` redirect, the visible URL stays `games.elijah.run`.

Cloudflare (proxied) sits in front of the `elijah.run` zone, so there are two
ways to do this.

## Option A — Worker + Service Binding (DEPLOYED)

`games-legacy-host.worker.js` is a reverse proxy: it swaps the host to
`pages.elijah.run`, prefixes the path with `/portfolios`, and forwards the
request. Because the built site uses relative asset paths, a flat 1:1 prefix is
sufficient.

**CRITICAL — Service Binding:** `pages.elijah.run` is *itself* a Worker on this
zone (`pages.elijah.run/*` → script `pages`). A same-zone Worker→Worker call via
global `fetch()` is **not** routed through that Worker (Cloudflare loop
prevention) — it falls through to a non-existent origin and returns **522**. So
the proxy calls the `pages` Worker through a **Service Binding** (`env.PAGES`),
declared in `wrangler.toml`:

```toml
[[services]]
binding = "PAGES"
service = "pages"
```

Deploy with the flake's dev shell:

```sh
nix develop -c wrangler deploy -c deploy/wrangler.toml
```

DNS: `games.elijah.run` needs a **proxied** (orange-cloud) record so requests
reach the Worker route — a placeholder `AAAA games → 100::` (the Worker
intercepts; the origin is never contacted). It's managed in Terraform at
`../../editor/infra/games-elijah-run.tf` (the only TF root wired to the
elijah.run zone); `deploy/dns.tf` here is just a pointer to it.

## Option B — Declarative, no Worker (`cloudflare_ruleset` in Terraform)

There is no single "redirect/rewrite" Cloudflare resource, but
`cloudflare_ruleset` covers it via phases. A transparent serve needs an **origin
rule** (override the origin host) plus a **URL-rewrite transform** (prefix the
path). Redirects (301/302) would instead use the
`http_request_dynamic_redirect` phase — not what we want here.

```hcl
# Change the origin host: send games.elijah.run requests to pages.elijah.run.
resource "cloudflare_ruleset" "games_origin" {
  zone_id = var.elijah_run_zone_id
  name    = "games -> pages origin"
  kind    = "zone"
  phase   = "http_request_origin"

  rules {
    expression = "(http.host eq \"games.elijah.run\")"
    action     = "route"
    action_parameters {
      host_header = "pages.elijah.run"
      origin { host = "pages.elijah.run" }
    }
  }
}

# Prefix the path with /portfolios so the sub-path bundle is served.
resource "cloudflare_ruleset" "games_rewrite" {
  zone_id = var.elijah_run_zone_id
  name    = "games -> /portfolios rewrite"
  kind    = "zone"
  phase   = "http_request_transform"

  rules {
    expression = "(http.host eq \"games.elijah.run\")"
    action     = "rewrite"
    action_parameters {
      uri {
        path { expression = "concat(\"/portfolios\", http.request.uri.path)" }
      }
    }
  }
}
```

The Worker (Option A) can also be managed in Terraform via
`cloudflare_workers_script` + `cloudflare_workers_route` if you prefer keeping
the JS but declaring the binding.

> Adjust to your actual origin setup — Option B assumes `pages.elijah.run` is a
> reachable origin within/behind the same Cloudflare zone.

## Caveat: other paths on `games.elijah.run`

Both options rewrite **every** path under `games.elijah.run` into the portfolios
bundle. Nothing else is expected to live there: the Set game (formerly
`games.elijah.run/set/`) now lives at `pages.elijah.run/set/`, and
`portfolios.toml` links it there directly. The legacy `games.elijah.run/set/`
URL will 404 after cutover — repoint any external links to `pages.elijah.run/set/`.

If you later add sibling content on `games.elijah.run`, narrow the route/expression
to exclude those paths (or add a pass-through branch in the Worker).
