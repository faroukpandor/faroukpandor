# Victor Hugo — maintained fork

A working copy of [netlify-templates/victor-hugo](https://github.com/netlify-templates/victor-hugo),
the Hugo + Webpack boilerplate that Netlify deprecated in December 2021.

**Upstream is archived**, which means it is read-only: no pull requests, no issue comments,
no pushes. It last received a commit on 2021-12-23 and still has 25 open issues. This fork
exists because the template could no longer be installed or built, and 362 forks inherited
that breakage.

---

## Why this fork exists

Upstream does not install on any current Node runtime. On Node 22:

```
npm error gyp ERR! configure error
npm error gyp ERR! System Linux
npm error gyp ERR! command "node" ".../node-gyp/bin/node-gyp.js" "rebuild" ...
npm error Build failed with error code: 1
```

`node-sass@6` is a native module with no prebuilt binary beyond Node 16, so `npm install`
falls back to compiling it — which needs a Python 2 toolchain that no longer ships anywhere.

Two further bugs stop the build even after the install succeeds. Both are in this fork's
history because nobody had reached them: they sit *behind* the install failure, so anyone
who fixed sass by hand hit these next and stopped.

---

## What was fixed

| # | File | Change | Symptom it removes |
|---|---|---|---|
| 1 | `package.json` | `node-sass@^6` → `sass@^1.80` | `npm install` fails to compile a native module on Node ≥ 17. dart-sass is pure JavaScript. |
| 2 | `.babelrc` | Removed both `plugins` entries | **`Cannot find package '@babel/plugin-syntax-object-rest-spread'`** — the build referenced two Babel plugins that were never declared as dependencies. `@babel/preset-env` has handled object rest/spread natively since Babel 7. |
| 3 | `webpack.common.js` | Dropped the `json-loader` rule | Any `.json` import would throw — `json-loader` isn't a dependency, and Webpack 5 parses JSON natively. |
| 4 | `webpack.common.js` | `name: "name=/[hash].[ext]"` → `"[hash].[ext]"` | The `name=` prefix is not valid `file-loader` syntax; it was being written into the emitted filename. |
| 5 | `package.json` | `sass-loader@^12` → `^16` | `Deprecation: The legacy JS API is deprecated and will be removed in Dart Sass 2.0.0` on every build. |
| 6 | `webpack.prod.js` | `[hash]` → `[contenthash]` | `DeprecationWarning: [hash] is now [fullhash]` — and `contenthash` gives correct long-term caching. |
| 7 | `.nvmrc` | `16` → `22` | The pinned version is the one that cannot install the project. |

Items 2 and 3 are not reported in any upstream issue. They were found by building the
project after item 1.

---

## Verified

```
$ node -v
v22.22.3

$ npm install
added 1091 packages in 42s

$ npm run build:webpack
webpack 5.112.0 compiled successfully in 2045 ms

$ find dist -type f
dist/fonts/.keep
dist/index.html
dist/main.5751e.css
dist/main.d9a4c.js
```

No warnings, no native compilation, content-hashed assets, and `site/data/webpack.json`
written with the filenames Hugo reads.

### What is *not* verified here

`npm run build:hugo` was not run in the environment this fork was prepared in, because
`hugo-bin`'s postinstall downloads a Hugo release binary from GitHub and the sandbox
intercepted the TLS handshake (`unable to verify the first certificate`). That is an
environment limitation, not a defect — on a normal machine or in CI it resolves. The Hugo
input (`site/`) is untouched from upstream.

If you want a fully verified end-to-end build, run `npm run build` once on your own machine
and confirm it writes `dist/`.

---

## Usage

```bash
npm install        # no native toolchain required
npm start          # Hugo + webpack dev servers, hot reload
npm run build      # production build into dist/
```

Node 18 or newer. No `node-sass`, no Python, no C++ compiler.

## Deploying

`netlify.toml` is unchanged from upstream. The build runs `npm run build` and publishes
`dist/`.

## Status and intent

This is a fork, not a revival. Upstream is archived deliberately — Netlify recommends
[Hyas](https://github.com/h-enk/hyas) for new projects, and so would I. The purpose of this
repository is narrower: to leave a copy of the template that actually installs, for anyone
who is keeping one of the 362 existing forks alive.

If you are starting a new project, use Hyas. If you are maintaining an old Victor Hugo
site and `npm install` just broke, start here.

## Contributing

Upstream cannot accept contributions — it is archived and read-only. If you want to improve
Victor Hugo itself, the practical options are:

1. Open a discussion on a maintained fork.
2. Take the patch above to [h-enk/hyas](https://github.com/h-enk/hyas), which is actively
   maintained.

## License

MIT, inherited from upstream. See [LICENSE](LICENSE).
