resource "cloudflare_worker" "oauth" {
  account_id = var.cloudflare_account_id
  name       = "blog-editor-oauth"

  subdomain = {
    enabled = true
  }
}

resource "cloudflare_worker_version" "oauth" {
  account_id         = var.cloudflare_account_id
  worker_id          = cloudflare_worker.oauth.id
  compatibility_date = "2024-01-01"
  main_module        = "index.js"

  modules = [
    {
      name         = "index.js"
      content_type = "application/javascript+module"
      content_file = "${path.module}/../worker/build/index.js"
    },
    {
      name         = "index_bg.wasm"
      content_type = "application/wasm"
      content_file = "${path.module}/../worker/build/index_bg.wasm"
    },
  ]

  bindings = [
    {
      name = "GITHUB_CLIENT_ID"
      type = "secret_text"
      text = var.github_client_id
    },
    {
      name = "GITHUB_CLIENT_SECRET"
      type = "secret_text"
      text = var.github_client_secret
    },
  ]
}

resource "cloudflare_workers_deployment" "oauth" {
  account_id  = var.cloudflare_account_id
  script_name = cloudflare_worker.oauth.name
  strategy    = "percentage"

  versions = [{
    percentage = 100
    version_id = cloudflare_worker_version.oauth.id
  }]
}
