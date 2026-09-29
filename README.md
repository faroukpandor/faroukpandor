# 🦀 Dev Toolbox

A free, privacy-first developer toolbox that runs **entirely in your browser** —
built in **Rust**, compiled to **WebAssembly**. Nothing you type is ever sent to a server.

Live tools:
- 🔑 Password generator
- 🔤 Base64 encode / decode
- 🧾 JSON formatter (pretty-print / minify)

More tools will be added over time (see [Roadmap](#roadmap)).

> The previous Hugo/Victor-Hugo-based site has been archived in
> [`legacy-hugo-site/`](./legacy-hugo-site) for reference and is no longer maintained.

## Why this architecture?

Because the whole app is static HTML/CSS/WASM with **no backend**, it can be hosted
for free, indefinitely, with no credit card, no request limits, and no cold starts —
on **Cloudflare Pages**. See [this discussion](.) for the reasoning behind picking a
100%-client-side Rust app over a server-based one for long-term free hosting.

## Local development

Install prerequisites once:

```bash
# Install Rust: https://rustup.rs
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Then run a local dev server with hot reload:

```bash
trunk serve --open
```

Build a production bundle into `dist/`:

```bash
trunk build --release
```

## Deploying to Cloudflare Pages (free, forever)

1. Push this repo to GitHub (already done if you're reading this on GitHub).
2. Go to the [Cloudflare dashboard](https://dash.cloudflare.com/) → **Workers & Pages** → **Create** → **Pages** → **Connect to Git**.
3. Select this repository, then set the build config:
   - **Framework preset**: `None`
   - **Build command**: `curl https://sh.rustup.rs -sSf | sh -s -- -y && source "$HOME/.cargo/env" && rustup target add wasm32-unknown-unknown && cargo install trunk && trunk build --release`
   - **Build output directory**: `dist`
4. Deploy. You'll get a free `your-project.pages.dev` URL immediately, with free SSL
   and no bandwidth/request caps for normal traffic. You can later attach your own
   custom domain for free too, if you own one.

Every push to your default branch will auto-redeploy.

### Continuous integration

`.github/workflows/ci.yml` builds the project on every push/PR using GitHub Actions
(which has full internet access), so you get a build-status check even before
Cloudflare builds it. `.github/workflows/deploy-cloudflare-pages.yml` is an optional
alternative that deploys via `wrangler` from GitHub Actions instead of Cloudflare's
own Git integration — only needed if you prefer that flow.

## Monetization ideas (optional, still zero-cost to host)

- Non-intrusive ads (e.g. Google AdSense / EthicalAds) — just a script tag, no backend.
- A "Pro" tier (e.g. remove ads, unlock more tools/batch mode) sold as a one-time
  or recurring payment via a hosted payment link (Stripe Payment Links or Lemon
  Squeezy) — no backend required to accept payment.
- More free tools = more organic/SEO traffic over time.

## Also in this repo: a lead-magnet validation page

`leadmagnet/` is a separate, unrelated micro-project: a zero-backend landing
page for validating a completely different idea (trading a free digital
asset for a WhatsApp/email opt-in, to test demand before building a paid
subscription product). It deploys as its own free Cloudflare Pages project
from this same repo. See [`leadmagnet/README.md`](./leadmagnet/README.md).

## Roadmap

- [ ] QR code generator
- [ ] UUID generator
- [ ] Regex tester
- [ ] Unit converter
- [ ] Markdown previewer
- [ ] "Pro" unlock flow

## License

MIT — see [LICENSE](./LICENSE).
