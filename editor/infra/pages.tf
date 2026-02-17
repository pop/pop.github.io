resource "random_id" "pages_name" {
  byte_length = 8
}

resource "cloudflare_pages_project" "editor" {
  account_id        = var.cloudflare_account_id
  name              = random_id.pages_name.hex
  production_branch = "source"
}
