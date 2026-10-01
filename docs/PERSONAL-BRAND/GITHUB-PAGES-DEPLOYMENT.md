# GitHub Pages Deployment Architecture

## Primary URL

`https://faroukpandor.github.io/`

GitHub Pages supports user sites using the repository naming convention:

`<username>.github.io`

Therefore the canonical publishing repository should ultimately be:

`faroukpandor/faroukpandor.github.io`

## Migration strategy

Do not destroy or rename the existing `faroukpandor` repository until its history and useful content have been preserved and the replacement publishing repository is ready.

Recommended sequence:

1. Archaeology of `faroukpandor`.
2. Preserve historical content and Git history.
3. Establish the professional/portfolio information architecture.
4. Replace obsolete site infrastructure with a modern static-first implementation.
5. Create `faroukpandor.github.io`.
6. Publish the new site through GitHub Pages.
7. Validate URL, navigation, mobile UX, accessibility, SEO and performance.
8. Keep the source/development repository and deployment repository relationship documented.
9. Maintain a Cloudflare Pages-compatible build as fallback.

## Free-first requirements

The site must not require:

- paid hosting
- paid CMS
- paid analytics
- proprietary runtime services
- server-side application infrastructure for the core experience

Prefer:

- static generation
- GitHub
- GitHub Actions where appropriate
- open-source dependencies
- local/build-time data
- progressive enhancement
- portable Markdown/JSON/YAML data

## Security

Never place secrets, private contact databases or sensitive documents in the public repository.

GitHub Pages is a public web-hosting mechanism. The public repository/site must contain only information intended for publication.

## Custom domain later

A custom domain may be added later without redesigning the site architecture. GitHub Pages supports custom domains and HTTPS.

## Fallback

The deployment artifact must remain compatible with Cloudflare Pages.

If GitHub Pages becomes unsuitable, deployment should be switchable without rewriting the application.
