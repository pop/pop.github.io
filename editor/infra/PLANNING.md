# Deploy Blog Editor: OpenTofu + Wrangler

## Context

The blog editor (Yew/WASM app) needs two Cloudflare resources to go live:
1. **Cloudflare Worker** — OAuth token exchange endpoint (`blog-editor-oauth`)
2. **Cloudflare Pages** — static hosting for the WASM frontend

The GitHub OAuth App is created (Client ID: `Ov23lidCWWsvthYknofh`). The Client ID is already set in `login.rs`. The `WORKER_URL` in `auth.rs` is still empty and needs the deployed worker URL.

**Approach: Hybrid OpenTofu + Wrangler.** OpenTofu creates the infrastructure (Pages project). Wrangler handles code deployments (worker script + Pages assets) and worker secrets. This is pragmatic because the Cloudflare TF provider has poor support for multi-module Rust WASM workers (`shim.mjs` + `.wasm` binary).

## Changes

### 1. Add `opentofu` to `flake.nix`

**File:** `editor/flake.nix`
Add `opentofu` to the `buildInputs` list alongside `wrangler`.

### 2. Create `editor/infra/` with OpenTofu config

**`editor/infra/providers.tf`**
```hcl
terraform {
  required_version = ">= 1.6.0"
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.0"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}
```

**`editor/infra/variables.tf`**
- `cloudflare_api_token` (string, sensitive)
- `cloudflare_account_id` (string)
- `pages_project_name` (string, default `"blog-editor"`)

**`editor/infra/pages.tf`**
- `cloudflare_pages_project` resource — creates the Pages project for direct upload (no `source` block)

**`editor/infra/outputs.tf`**
- `pages_url` — `https://<project_name>.pages.dev`
- `pages_project_name` — for use with `wrangler pages deploy`

**`editor/infra/terraform.tfvars`** (gitignored, created manually by user)
- `cloudflare_api_token` and `cloudflare_account_id` values

### 3. Update `.gitignore`

**File:** `editor/.gitignore`
Add:
```
/infra/.terraform/
/infra/*.tfstate
/infra/*.tfstate.backup
/infra/terraform.tfvars
```

### 4. Deploy worker with Wrangler, set `WORKER_URL`

After `tofu apply` creates the Pages project:
1. `cd editor/worker && wrangler deploy` — deploys the worker, prints the URL
2. `wrangler secret put GITHUB_CLIENT_ID` — set to `Ov23lidCWWsvthYknofh`
3. `wrangler secret put GITHUB_CLIENT_SECRET` — set to the client secret
4. Set `WORKER_URL` in `editor/src/services/auth.rs:8` to the deployed worker URL

### 5. Build and deploy frontend

1. `trunk build --release` from `editor/`
2. `wrangler pages deploy dist/ --project-name=blog-editor`

## Files modified

| File | Action |
|---|---|
| `editor/flake.nix` | Add `opentofu` to buildInputs |
| `editor/.gitignore` | Add infra state/secrets patterns |
| `editor/infra/providers.tf` | New — provider config |
| `editor/infra/variables.tf` | New — input variables |
| `editor/infra/pages.tf` | New — Pages project resource |
| `editor/infra/outputs.tf` | New — output URLs |
| `editor/src/services/auth.rs` | Set `WORKER_URL` constant (after worker deploy) |

## Verification

1. `tofu init && tofu plan` — should show 1 resource to create (Pages project)
2. `tofu apply` — creates the Pages project on Cloudflare
3. `cd worker && wrangler deploy` — deploys worker, prints URL
4. `wrangler secret put GITHUB_CLIENT_ID` + `GITHUB_CLIENT_SECRET` — sets secrets
5. `trunk build --release` — builds frontend WASM bundle
6. `wrangler pages deploy dist/ --project-name=blog-editor` — deploys frontend
7. Visit the Pages URL, click "Login with GitHub" — should redirect to GitHub OAuth, then back with a token

## Note on secrets

The GitHub client secret was shared in this conversation. After deployment is verified, regenerate it in the GitHub OAuth App settings and update the worker secret with `wrangler secret put GITHUB_CLIENT_SECRET`.
