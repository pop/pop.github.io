# DNS record for games.elijah.run — the legacy hostname for the portfolios site.
#
# This proxied record just gives the hostname a Cloudflare edge presence. The
# "games-legacy-host" Worker (deployed out-of-band via wrangler from
# ../../portfolios/deploy/) owns a `games.elijah.run/*` route that intercepts
# every request and transparently serves pages.elijah.run/portfolios. The 100::
# target is an RFC 6666 black hole — never actually contacted.
#
# Lives here (not portfolios/) because this is the only Terraform root wired to
# the elijah.run zone + Cloudflare credentials.
resource "cloudflare_dns_record" "games" {
  zone_id = var.cloudflare_zone_id
  name    = "games"
  type    = "AAAA"
  content = "100::"
  proxied = true
  ttl     = 1
}
