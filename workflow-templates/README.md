# GitHub Actions workflow templates

These aren't active yet. The automated agent that pushes to this repo currently
only has permission to modify regular files — GitHub blocks any bot/app that
lacks the `workflows` permission from creating or editing files under
`.github/workflows/`, so these live here instead.

To enable them yourself (takes ~30 seconds, one-time):

1. In the GitHub web UI (or locally with `git`), create the folder `.github/workflows/`.
2. Copy `ci.yml` and/or `deploy-cloudflare-pages.yml` from this folder into it.
3. Commit directly on github.com, or `git add .github/workflows && git commit && git push`.

- **ci.yml** — builds the Rust/WASM app with Trunk on every push/PR, so you get a
  pass/fail check without needing Rust installed locally.
- **deploy-cloudflare-pages.yml** — optional, only needed if you'd rather deploy via
  GitHub Actions + `wrangler` instead of Cloudflare's own Git integration (which is
  the simpler default — see the main [README](../README.md)).
