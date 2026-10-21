# faroukpandor

The source repository for **[faroukpandor.github.io](https://faroukpandor.github.io/)** — a
portfolio hub that grades its own claims.

No framework. No build step. No external requests. HTML, CSS and about forty lines of
JavaScript, deployed straight from `site/`.

---

## Why this repository looks like this

It was a fork of Netlify's **Victor Hugo** boilerplate — a Hugo + Webpack + node-sass stack
that had already been deprecated upstream and could no longer install on a current Node
runtime. The homepage still said *"Hooray 🎉 — you've built this with Victor-Hugo"*.

Rather than patch a dead toolchain, the site was rebuilt as static HTML and the old stack
was archived intact at [`archive/legacy-victor-hugo/`](archive/legacy-victor-hugo/). It is
kept because the migration was supposed to be reversible, not because anyone should run it.

The 2025 *Strategy for Monetizing Passions* document — the one piece of genuinely personal
material in the old repository — is preserved at
[`archive/2025-strategy-monetizing/`](archive/2025-strategy-monetizing/).

## Structure

```
site/                  the published site — this is the deployable artefact
  index.html           the whole page
  assets/css/main.css  design system, hand-written
  assets/js/main.js    work filter + footer year. Nothing else.
  404.html
  robots.txt  sitemap.xml  .nojekyll
archive/               superseded infrastructure, kept for reversibility
```

Operating records — the charter, the Drive and Gmail operating systems, the deployment and
commercial registers, client files — are **not** in this repository. They live in private
storage because they contain client names, disputed invoices and unverified claims that
have no business being publicly indexed. Publishing those alongside a portfolio would
undermine the very evidence standard the portfolio claims to hold.

## Run it locally

Any static server. There is nothing to compile.

```bash
cd site && python3 -m http.server 8000
```

## Deploy

`.github/workflows/pages.yml` publishes `site/` to GitHub Pages on every push that touches
it. The artefact is plain static files, so the same directory can be dropped on Cloudflare
Pages, Netlify or S3 with no configuration change — which is the point.

To use a custom domain later, add a `CNAME` file to `site/` and point the DNS record; no
restructuring required.

## The editorial rule

Every claim on the site carries a provenance level:

| Grade | Meaning |
|---|---|
| **Verified** | Loaded and checked by hand; the public URL exists today |
| **Documented** | A record exists and can be produced on request |
| **Self-reported** | True to the author's knowledge, not independently evidenced |
| **Historical** | Real once; not current, and never presented as operating |

Client work is attributed, never claimed. Where a client owns the brand or the
intellectual property, the page says so. Where an enterprise has no published price, the
page says that too rather than inventing one.

The habit being avoided: a portfolio of confident adjectives with nothing behind them.

## What is deliberately absent

No client names beyond those who have agreed to be credited, no invoice data, no dispute
records, no unverified claims, and no credentials. The site describes work; it does not
publish other people's commercial business.

## License

Site content © Farouk Gerald Pandor. The inherited boilerplate remains under the MIT
licence in [`LICENSE`](LICENSE).
