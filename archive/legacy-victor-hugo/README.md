# Victor Hugo (archived)

This directory holds the site infrastructure this repository was originally forked
with: Netlify's **Victor Hugo** boilerplate — Hugo for templating, Webpack 5 for assets,
node-sass for styles.

## Why it is here and not deleted

`docs/PERSONAL-BRAND/README.md` (private records, not in this repository) set the rule:
*preserve useful content and history → replace obsolete infrastructure deliberately → never
perform a blind dependency upgrade or wholesale deletion.* Moving the stack here satisfies
both halves. The history remains intact; the root is clear.

## Why it is not used

- **Upstream is deprecated.** Netlify stopped maintaining the template and recommends Hyas
  instead.
- **It will not install.** `node-sass@6` supports Node 16 at most and needs a Python 2
  toolchain. The repository now runs on Node 22.
- **It built the wrong site.** The Hugo build ran from `site/`, but the personal content
  lived in root `content/`, which Hugo never read. Two `config.toml` files disagreed about
  which site existed.
- **The theme was never present.** `themes/hugo-theme-minimal` was an uninitialised git
  submodule pointing at `calintat/minimal` — an empty directory. Nothing could have built.

The replacement is static HTML, CSS and a small amount of JavaScript in `site/`, with no
compile step. Nothing in this folder is referenced by the current site.

## If you ever want it back

The full history is in git:

```bash
git log --follow -- archive/legacy-victor-hugo/package.json
```

Reconstructing it would mean restoring these files to the root, initialising the theme
submodule, and pinning Node 16 — after which it would still build the boilerplate
homepage, not a portfolio.
