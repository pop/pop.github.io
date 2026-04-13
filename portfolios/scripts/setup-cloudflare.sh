#!/usr/bin/env bash
set -euo pipefail

PROJECT="games-elijah-run"
DOMAIN="games.elijah.run"

echo "==> Creating Cloudflare Pages project: $PROJECT"
wrangler pages project create "$PROJECT" --production-branch main

echo "==> Adding custom domain: $DOMAIN"
wrangler pages domain add "$DOMAIN" --project-name "$PROJECT"

echo "==> Done. DNS: add a CNAME for $DOMAIN → ${PROJECT}.pages.dev"
echo "    (If $DOMAIN is already on Cloudflare, this is handled automatically.)"
