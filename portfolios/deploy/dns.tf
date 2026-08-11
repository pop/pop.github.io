# NOT a runnable root — pointer only.
#
# The proxied DNS record for games.elijah.run is managed in the actual Terraform
# root that owns the elijah.run zone + Cloudflare credentials:
#
#     ../../editor/infra/games-elijah-run.tf   (resource cloudflare_dns_record.games)
#
# It is a placeholder `AAAA games -> 100::`, proxied; the games-legacy-host
# Worker route (see wrangler.toml) intercepts every request, so the origin is
# never contacted. Do NOT redefine the record here — a second definition in a
# separate root would fight the editor/infra state for the same record.
