# Deploy Blog Editor: OpenTofu Infrastructure

## Overview

The blog editor (Yew/WASM app) uses two Cloudflare resources:
1. **Cloudflare Worker** — OAuth token exchange endpoint
2. **Cloudflare Pages** — static hosting for the WASM frontend

All infrastructure is managed with OpenTofu. Secrets live in `terraform.tfvars` (gitignored).

## Resources

| Resource | Type | Purpose |
|---|---|---|
| `cloudflare_worker.oauth` | Worker | Creates the worker with workers.dev subdomain enabled |
| `cloudflare_worker_version.oauth` | Worker Version | Uploads JS + WASM modules and configures secret bindings |
| `cloudflare_workers_deployment.oauth` | Deployment | Routes 100% traffic to the current version |
| `cloudflare_pages_project.editor` | Pages Project | Hosts the frontend (direct upload, no git integration) |
| `random_id.pages_name` | Random ID | Generates an opaque project name for the Pages URL |

## Setup

1. Create a GitHub OAuth App and note the client ID and secret
2. Create a Cloudflare API token with Workers and Pages edit permissions
3. Create `editor/infra/terraform.tfvars`:
   ```hcl
   cloudflare_api_token  = "..."
   cloudflare_account_id = "..."
   github_client_id      = "..."
   github_client_secret  = "..."
   ```
4. Build the worker: `cd editor/worker && worker-build --release`
5. Apply infrastructure: `cd editor/infra && tofu init && tofu apply`
6. Set `GITHUB_CLIENT_ID` in `editor/src/components/login.rs`
7. Set `WORKER_URL` in `editor/src/services/auth.rs` to the worker URL from `tofu output`

## Frontend deployment

After infrastructure is applied:
1. `trunk build --release` from `editor/`
2. `wrangler pages deploy dist/ --project-name=$(tofu -chdir=infra output -raw pages_project_name)`

## Updating the worker

Rebuild and re-apply:
1. `cd editor/worker && worker-build --release`
2. `cd editor/infra && tofu apply`

OpenTofu detects content changes via SHA256 hashes and creates a new version + deployment automatically.

## Note on secrets

All sensitive values (`cloudflare_api_token`, `github_client_id`, `github_client_secret`) are in `terraform.tfvars` which is gitignored. The GitHub client ID is also embedded in the frontend source (required for the OAuth redirect flow). After initial setup, rotate the GitHub client secret and update via `tofu apply`.
