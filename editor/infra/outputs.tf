output "pages_project_name" {
  description = "Pages project name (use with: wrangler pages deploy dist/ --project-name=<this>)"
  value       = cloudflare_pages_project.editor.name
}

output "pages_url" {
  description = "Default Pages URL"
  value       = "https://${cloudflare_pages_project.editor.name}.pages.dev"
}

output "worker_url" {
  description = "OAuth worker URL"
  value       = "https://${cloudflare_worker.oauth.name}.homeworkbad.workers.dev"
}
