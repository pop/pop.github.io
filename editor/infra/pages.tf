resource "random_id" "pages_name" {
  byte_length = 8
}

resource "cloudflare_pages_project" "editor" {
  account_id        = var.cloudflare_account_id
  name              = random_id.pages_name.hex
  production_branch = "source"
}

resource "cloudflare_pages_domain" "editor" {
  account_id   = var.cloudflare_account_id
  project_name = cloudflare_pages_project.editor.name
  name         = "editor.elijah.run"
}

resource "cloudflare_dns_record" "editor" {
  zone_id = var.cloudflare_zone_id
  name    = "editor"
  type    = "CNAME"
  content = "${cloudflare_pages_project.editor.name}.pages.dev"
  proxied = true
  ttl     = 1
}
