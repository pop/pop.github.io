# Deploy to Production
1. Run `cargo build --release` to verify clean build
2. Deploy using OpenTofu/Terraform config (NOT raw wrangler commands)
3. Ensure deployment targets PRODUCTION environment, not preview
4. Verify the deployment is live
5. Commit any deployment-related changes
